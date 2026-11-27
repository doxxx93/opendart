//! 최대주주 현황 (`hyslrSttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "hyslrSttus.json";

/// One row of 최대주주 현황: shares held by the largest shareholder and related
/// parties.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct LargestShareholder {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 법인명
    pub corp_name: String,
    /// 성명
    #[serde(default)]
    pub nm: String,
    /// 관계
    #[serde(default)]
    pub relate: String,
    /// 주식 종류
    #[serde(default)]
    pub stock_knd: String,
    /// 기초 소유 주식 수
    #[serde(default, deserialize_with = "decode::number")]
    pub bsis_posesn_stock_co: Option<Number>,
    /// 기초 소유 주식 지분 율
    #[serde(default, deserialize_with = "decode::number")]
    pub bsis_posesn_stock_qota_rt: Option<Number>,
    /// 기말 소유 주식 수
    #[serde(default, deserialize_with = "decode::number")]
    pub trmend_posesn_stock_co: Option<Number>,
    /// 기말 소유 주식 지분 율
    #[serde(default, deserialize_with = "decode::number")]
    pub trmend_posesn_stock_qota_rt: Option<Number>,
    /// 비고
    #[serde(default)]
    pub rm: String,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 최대주주 현황 (`hyslrSttus.json`): shares held by the largest shareholder and
    /// related parties.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn hyslr_sttus(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<LargestShareholder>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/hyslrSttus.json");
        let rows: Vec<LargestShareholder> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 5);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(
            rows[0].bsis_posesn_stock_co.as_ref().map(Number::as_str),
            Some("508,157,148")
        );
    }
}
