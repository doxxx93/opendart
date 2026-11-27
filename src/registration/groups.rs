//! Decoding of the grouped replies: `{"group": [{"title": .., "list": [..]}]}`.

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
    list: serde_json::Value,
}

/// A registration statement type built from the groups of a reply.
pub(super) trait FromGroups: Default {
    fn from_groups(endpoint: &'static str, groups: &mut [Group]) -> Result<Self, Error>;
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
        let list: Vec<T> = serde_json::from_value(group.list.take())
            .map_err(|err| Error::decode(endpoint, err))?;
        rows.extend(list);
    }
    Ok(rows)
}

/// Decodes a grouped reply; [`Status::NoData`] gives empty groups.
pub(super) fn parse<T: FromGroups>(endpoint: &'static str, body: &[u8]) -> Result<T, Error> {
    match decode::json::<Groups>(endpoint, body) {
        Ok(mut groups) => T::from_groups(endpoint, &mut groups.group),
        Err(Error::Api {
            status: Status::NoData,
            ..
        }) => Ok(T::default()),
        Err(err) => Err(err),
    }
}

impl Client {
    pub(super) async fn registration<T: FromGroups>(
        &self,
        endpoint: &'static str,
        request: &DateRangeRequest,
    ) -> Result<T, Error> {
        let body = self.get(endpoint, &request.query()).await?;
        parse(endpoint, &body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registration::RestructuringRegistration;

    #[test]
    fn no_data_is_empty() {
        let body = include_bytes!("../../tests/fixtures/list/no_data.json");
        let statement: RestructuringRegistration = parse("mgRs.json", body).unwrap();
        assert_eq!(statement, RestructuringRegistration::default());
    }

    #[test]
    fn unknown_groups_are_ignored() {
        let body =
            br#"{"status":"000","message":"","group":[{"title":"new group","list":[{"x":"1"}]}]}"#;
        let statement: RestructuringRegistration = parse("mgRs.json", body).unwrap();
        assert!(statement.general.is_empty());
    }
}
