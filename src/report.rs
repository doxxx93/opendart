//! The request shared by the periodic report endpoints (`DS002`, `DS003`).

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

impl Client {
    /// Fetches a `list` of rows for one report; no data is an empty list.
    pub(crate) async fn report_rows<T: DeserializeOwned>(
        &self,
        endpoint: &'static str,
        request: &ReportRequest,
    ) -> Result<Vec<T>, Error> {
        let body = self.get(endpoint, &request.query()).await?;
        decode::rows(endpoint, &body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
