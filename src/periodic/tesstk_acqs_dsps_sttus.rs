//! 자기주식 취득 및 처분 현황 (`tesstkAcqsDspsSttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "tesstkAcqsDspsSttus.json";

/// One row of 자기주식 취득 및 처분 현황: acquisition and disposal of treasury stock.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct TreasuryStock {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 법인명
    pub corp_name: String,
    /// 취득방법 대분류
    #[serde(default)]
    pub acqs_mth1: String,
    /// 취득방법 중분류
    #[serde(default)]
    pub acqs_mth2: String,
    /// 취득방법 소분류
    #[serde(default)]
    pub acqs_mth3: String,
    /// 주식 종류
    #[serde(default)]
    pub stock_knd: String,
    /// 기초 수량
    #[serde(default, deserialize_with = "decode::number")]
    pub bsis_qy: Option<Number>,
    /// 변동 수량 취득
    #[serde(default, deserialize_with = "decode::number")]
    pub change_qy_acqs: Option<Number>,
    /// 변동 수량 처분
    #[serde(default, deserialize_with = "decode::number")]
    pub change_qy_dsps: Option<Number>,
    /// 변동 수량 소각
    #[serde(default, deserialize_with = "decode::number")]
    pub change_qy_incnr: Option<Number>,
    /// 기말 수량
    #[serde(default, deserialize_with = "decode::number")]
    pub trmend_qy: Option<Number>,
    /// 비고
    #[serde(default)]
    pub rm: String,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 자기주식 취득 및 처분 현황 (`tesstkAcqsDspsSttus.json`): acquisition and disposal
    /// of treasury stock.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn tesstk_acqs_dsps_sttus(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<TreasuryStock>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/tesstkAcqsDspsSttus.json");
        let rows: Vec<TreasuryStock> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(
            rows[0].bsis_qy.as_ref().map(Number::as_str),
            Some("29,700,000")
        );
    }
}
