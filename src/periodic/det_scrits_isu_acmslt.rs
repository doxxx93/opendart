//! 채무증권 발행실적 (`detScritsIsuAcmslt.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "detScritsIsuAcmslt.json";

/// One row of 채무증권 발행실적: debt securities issued.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct DebtIssuance {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 법인명
    pub corp_name: String,
    /// 발행회사
    #[serde(default)]
    pub isu_cmpny: String,
    /// 증권종류
    #[serde(default)]
    pub scrits_knd_nm: String,
    /// 발행방법
    #[serde(default)]
    pub isu_mth_nm: String,
    /// 발행일자
    #[serde(default)]
    pub isu_de: String,
    /// 권면(전자등록)총액
    #[serde(default, deserialize_with = "decode::number")]
    pub facvalu_totamt: Option<Number>,
    /// 이자율
    #[serde(default, deserialize_with = "decode::number")]
    pub intrt: Option<Number>,
    /// 평가등급(평가기관)
    #[serde(default)]
    pub evl_grad_instt: String,
    /// 만기일
    #[serde(default)]
    pub mtd: String,
    /// 상환여부
    #[serde(default)]
    pub repy_at: String,
    /// 주관회사
    #[serde(default)]
    pub mngt_cmpny: String,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 채무증권 발행실적 (`detScritsIsuAcmslt.json`): debt securities issued.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn det_scrits_isu_acmslt(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<DebtIssuance>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/detScritsIsuAcmslt.json");
        let rows: Vec<DebtIssuance> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(
            rows[0].facvalu_totamt.as_ref().map(Number::as_str),
            Some("143,490,000,000")
        );
    }
}
