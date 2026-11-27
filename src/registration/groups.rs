//! Decoding of the grouped replies: `{"group": [{"title": .., "list": [..]}]}`.
//!
//! A reply holds every statement filed in the date range, each group mixing
//! the rows of all of them; the rows are sorted back into one value per
//! statement by their `rcept_no`.

use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::{Client, DateRangeRequest, Error, Status, decode};

#[derive(Deserialize)]
struct Groups {
    #[serde(default)]
    group: Vec<Group>,
}

/// One group of a reply, its rows still undecoded.
#[derive(Deserialize)]
pub(super) struct Group {
    #[serde(default)]
    title: String,
    #[serde(default)]
    list: Vec<serde_json::Value>,
}

/// A registration statement type, built from the rows that carry its
/// receipt number.
pub(super) trait Statement: Sized {
    fn new(rcept_no: &str) -> Self;
    fn rcept_no(&self) -> &str;
    /// Sorts the rows of every group into statements, in order of first
    /// appearance.
    fn from_groups(endpoint: &'static str, groups: &mut [Group]) -> Result<Vec<Self>, Error>;
}

/// The rows of every group titled `title`; none if the reply has no such
/// group. Groups with other titles are left for other fields, and titles
/// this crate does not know are ignored.
pub(super) fn take<T: DeserializeOwned>(
    endpoint: &'static str,
    groups: &mut [Group],
    title: &str,
) -> Result<Vec<T>, Error> {
    let mut rows = Vec::new();
    for group in groups.iter_mut().filter(|group| group.title == title) {
        for value in std::mem::take(&mut group.list) {
            rows.push(serde_json::from_value(value).map_err(|err| Error::decode(endpoint, err))?);
        }
    }
    Ok(rows)
}

/// The statement with `rcept_no`, added at the end if it is new.
pub(super) fn statement<'a, S: Statement>(statements: &'a mut Vec<S>, rcept_no: &str) -> &'a mut S {
    let index = match statements.iter().position(|s| s.rcept_no() == rcept_no) {
        Some(index) => index,
        None => {
            statements.push(S::new(rcept_no));
            statements.len() - 1
        }
    };
    &mut statements[index]
}

/// Decodes a grouped reply; [`Status::NoData`] gives no statements.
pub(super) fn parse<S: Statement>(endpoint: &'static str, body: &[u8]) -> Result<Vec<S>, Error> {
    match decode::json::<Groups>(endpoint, body) {
        Ok(mut groups) => S::from_groups(endpoint, &mut groups.group),
        Err(Error::Api {
            status: Status::NoData,
            ..
        }) => Ok(Vec::new()),
        Err(err) => Err(err),
    }
}

impl Client {
    pub(super) async fn registration<S: Statement>(
        &self,
        endpoint: &'static str,
        request: &DateRangeRequest,
    ) -> Result<Vec<S>, Error> {
        let body = self.get(endpoint, &request.query()).await?;
        parse(endpoint, &body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registration::RestructuringRegistration;

    fn statements(body: &[u8]) -> Vec<RestructuringRegistration> {
        parse("mgRs.json", body).unwrap()
    }

    #[test]
    fn no_data_is_empty() {
        assert!(statements(include_bytes!("../../tests/fixtures/list/no_data.json")).is_empty());
    }

    #[test]
    fn unknown_groups_and_missing_lists_are_ignored() {
        let body = r#"{"status":"000","message":"","group":[{"title":"new group","list":[{"x":"1"}]},{"title":"일반사항"}]}"#;
        assert!(statements(body.as_bytes()).is_empty());
    }
}
