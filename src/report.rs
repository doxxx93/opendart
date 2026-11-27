//! The requests shared by the periodic report endpoints (`DS002`, `DS003`).

use serde::de::DeserializeOwned;

use crate::client::Query;
use crate::{Client, Error, decode};

/// Which periodic report to read (`reprt_code`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ReportCode {
    /// `11013`: 1분기보고서 (first quarter).
    FirstQuarter,
    /// `11012`: 반기보고서 (half year).
    HalfYear,
    /// `11014`: 3분기보고서 (third quarter).
    ThirdQuarter,
    /// `11011`: 사업보고서 (annual report).
    Annual,
}

impl ReportCode {
    /// The raw code, e.g. `"11011"`.
    pub fn code(self) -> &'static str {
        match self {
            ReportCode::FirstQuarter => "11013",
            ReportCode::HalfYear => "11012",
            ReportCode::ThirdQuarter => "11014",
            ReportCode::Annual => "11011",
        }
    }
}

/// One company's periodic report: `corp_code`, `bsns_year` and
/// `reprt_code`, the parameters every [`periodic`](crate::periodic)
/// endpoint takes.
///
/// ```
/// use opendart::{ReportCode, ReportRequest};
///
/// // Samsung Electronics, annual report for the 2025 fiscal year.
/// let request = ReportRequest::new("00126380", 2025, ReportCode::Annual);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReportRequest {
    corp_code: String,
    bsns_year: i16,
    reprt_code: ReportCode,
}

impl ReportRequest {
    /// A request for the report of `corp_code` (8 digits) for fiscal year
    /// `bsns_year`. OpenDART has data from 2015 on.
    pub fn new(corp_code: impl Into<String>, bsns_year: i16, reprt_code: ReportCode) -> Self {
        ReportRequest {
            corp_code: corp_code.into(),
            bsns_year,
            reprt_code,
        }
    }

    pub(crate) fn query(&self) -> Query {
        vec![
            ("corp_code", self.corp_code.clone()),
            ("bsns_year", self.bsns_year.to_string()),
            ("reprt_code", self.reprt_code.code().to_owned()),
        ]
    }
}

/// The same periodic report of several companies, for the multi-company
/// endpoints such as [`Client::fnltt_multi_acnt`].
///
/// OpenDART takes at most 100 companies per request; more fail with
/// [`Status::TooManyCompanies`](crate::Status::TooManyCompanies).
///
/// ```
/// use opendart::{MultiReportRequest, ReportCode};
///
/// // Samsung Electronics and SK hynix, annual reports for 2025.
/// let request = MultiReportRequest::new(["00126380", "00164779"], 2025, ReportCode::Annual);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MultiReportRequest {
    corp_codes: Vec<String>,
    bsns_year: i16,
    reprt_code: ReportCode,
}

impl MultiReportRequest {
    /// A request for the reports of `corp_codes` (8 digits each) for fiscal
    /// year `bsns_year`.
    pub fn new<I>(corp_codes: I, bsns_year: i16, reprt_code: ReportCode) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        MultiReportRequest {
            corp_codes: corp_codes.into_iter().map(Into::into).collect(),
            bsns_year,
            reprt_code,
        }
    }

    pub(crate) fn query(&self) -> Query {
        vec![
            ("corp_code", self.corp_codes.join(",")),
            ("bsns_year", self.bsns_year.to_string()),
            ("reprt_code", self.reprt_code.code().to_owned()),
        ]
    }
}

impl Client {
    /// Fetches the `list` of rows of a JSON endpoint; no data is an empty
    /// list.
    pub(crate) async fn rows<T: DeserializeOwned>(
        &self,
        endpoint: &'static str,
        query: &Query,
    ) -> Result<Vec<T>, Error> {
        let body = self.get(endpoint, query).await?;
        decode::rows(endpoint, &body)
    }

    /// [`rows`](Client::rows) for one company's report.
    pub(crate) async fn report_rows<T: DeserializeOwned>(
        &self,
        endpoint: &'static str,
        request: &ReportRequest,
    ) -> Result<Vec<T>, Error> {
        self.rows(endpoint, &request.query()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_request_joins_the_companies() {
        let request = MultiReportRequest::new(["00126380", "00164779"], 2025, ReportCode::Annual);
        assert_eq!(
            request.query()[0],
            ("corp_code", "00126380,00164779".to_owned())
        );
    }

    #[test]
    fn query_uses_the_wire_codes() {
        let request = ReportRequest::new("00126380", 2025, ReportCode::HalfYear);
        assert_eq!(
            request.query(),
            vec![
                ("corp_code", "00126380".to_owned()),
                ("bsns_year", "2025".to_owned()),
                ("reprt_code", "11012".to_owned()),
            ]
        );
    }
}
