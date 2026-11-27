//! 배당에 관한 사항 (`alotMatter.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "alotMatter.json";

/// One row of 배당에 관한 사항: dividends.
///
/// `se` names the item and its unit, such as `주당액면가액(원)`,
/// `(연결)당기순이익(백만원)` or `현금배당수익률(%)`, so `thstrm`, `frmtrm` and
/// `lwfr` hold won, millions of won or percentages depending on the row.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct Dividend {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 법인명
    pub corp_name: String,
    /// 구분
    #[serde(default)]
    pub se: String,
    /// 주식 종류
    #[serde(default)]
    pub stock_knd: String,
    /// 당기
    #[serde(default, deserialize_with = "decode::number")]
    pub thstrm: Option<Number>,
    /// 전기
    #[serde(default, deserialize_with = "decode::number")]
    pub frmtrm: Option<Number>,
    /// 전전기
    #[serde(default, deserialize_with = "decode::number")]
    pub lwfr: Option<Number>,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 배당에 관한 사항 (`alotMatter.json`): dividends.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn alot_matter(&self, request: &ReportRequest) -> Result<Vec<Dividend>, Error> {
        self.report_rows(ENDPOINT, request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_matches_the_guide() {
        let request = ReportRequest::new("00126380", 2025, crate::ReportCode::Annual);
        crate::guide_check::assert_documented(ENDPOINT, &request.query());
    }

    #[test]
    fn decodes_fixture() {
        let body = include_bytes!("../../tests/fixtures/periodic/alotMatter.json");
        let rows: Vec<Dividend> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 15);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(rows[0].thstrm.as_ref().map(Number::as_str), Some("100"));
    }
}
