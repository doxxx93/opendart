//! 이사·감사의 개인별 보수현황(5억원 이상) (`hmvAuditIndvdlBySttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "hmvAuditIndvdlBySttus.json";

/// One row of 이사·감사의 개인별 보수현황(5억원 이상): pay of each director and auditor paid
/// 500 million won or more.
///
/// `mendng_totamt_ct_incls_mendng` is prose about pay agreed but not yet
/// paid, not an amount, although the guide describes it as one.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct IndividualPay {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 법인명
    pub corp_name: String,
    /// 이름
    #[serde(default)]
    pub nm: String,
    /// 직위
    #[serde(default)]
    pub ofcps: String,
    /// 보수 총액
    #[serde(default, deserialize_with = "decode::number")]
    pub mendng_totamt: Option<Number>,
    /// 보수 총액 비 포함 보수
    #[serde(default)]
    pub mendng_totamt_ct_incls_mendng: String,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 이사·감사의 개인별 보수현황(5억원 이상) (`hmvAuditIndvdlBySttus.json`): pay of each
    /// director and auditor paid 500 million won or more.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn hmv_audit_indvdl_by_sttus(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<IndividualPay>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/hmvAuditIndvdlBySttus.json");
        let rows: Vec<IndividualPay> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(
            rows[0].mendng_totamt.as_ref().map(Number::as_str),
            Some("1,843,000,000")
        );
    }
}
