//! 미등기임원 보수현황 (`unrstExctvMendngSttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "unrstExctvMendngSttus.json";

/// One row of 미등기임원 보수현황: pay of unregistered executives.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct UnregisteredExecutivePay {
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
    /// 인원수
    #[serde(default, deserialize_with = "decode::number")]
    pub nmpr: Option<Number>,
    /// 연간급여 총액
    #[serde(default, deserialize_with = "decode::number")]
    pub fyer_salary_totamt: Option<Number>,
    /// 1인평균 급여액
    #[serde(default, deserialize_with = "decode::number")]
    pub jan_salary_am: Option<Number>,
    /// 비고
    #[serde(default)]
    pub rm: String,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 미등기임원 보수현황 (`unrstExctvMendngSttus.json`): pay of unregistered
    /// executives.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn unrst_exctv_mendng_sttus(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<UnregisteredExecutivePay>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/unrstExctvMendngSttus.json");
        let rows: Vec<UnregisteredExecutivePay> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(rows[0].nmpr.as_ref().map(Number::as_str), Some("989"));
    }
}
