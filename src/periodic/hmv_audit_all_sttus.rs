//! 이사·감사 전체의 보수현황(보수지급금액 - 이사·감사 전체) (`hmvAuditAllSttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "hmvAuditAllSttus.json";

/// One row of 이사·감사 전체의 보수현황(보수지급금액 - 이사·감사 전체): total pay of directors and
/// auditors.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct DirectorPayTotal {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 법인명
    pub corp_name: String,
    /// 인원수
    #[serde(default, deserialize_with = "decode::number")]
    pub nmpr: Option<Number>,
    /// 보수 총액
    #[serde(default, deserialize_with = "decode::number")]
    pub mendng_totamt: Option<Number>,
    /// 1인 평균 보수 액
    #[serde(default, deserialize_with = "decode::number")]
    pub jan_avrg_mendng_am: Option<Number>,
    /// 비고
    #[serde(default)]
    pub rm: String,
    /// 사업연도
    #[serde(default)]
    pub fscl_year: String,
    /// 보수총액 중 주식기준보상 지급액
    #[serde(default, deserialize_with = "decode::number")]
    pub stk_bsd_pd_mendng_totamt: Option<Number>,
    /// 주식매수선택권 행사가능수량
    #[serde(default, deserialize_with = "decode::number")]
    pub stk_opt_exrcsbl_qty: Option<Number>,
    /// 주식매수선택권 행사불가수량
    #[serde(default, deserialize_with = "decode::number")]
    pub stk_opt_unexrcsbl_qty: Option<Number>,
    /// 주식매수선택권 잔여금액
    #[serde(default, deserialize_with = "decode::number")]
    pub stk_opt_rmn_blce: Option<Number>,
    /// 그 외 주식기준 보상 미지급수량
    #[serde(default, deserialize_with = "decode::number")]
    pub othr_stk_bsd_cmpn_unpyd_qty: Option<Number>,
    /// 그 외 주식기준 보상 시장가치
    #[serde(default, deserialize_with = "decode::number")]
    pub othr_stk_bsd_cmpn_mkt_vl: Option<Number>,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 이사·감사 전체의 보수현황(보수지급금액 - 이사·감사 전체) (`hmvAuditAllSttus.json`): total pay
    /// of directors and auditors.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn hmv_audit_all_sttus(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<DirectorPayTotal>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/hmvAuditAllSttus.json");
        let rows: Vec<DirectorPayTotal> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(rows[0].nmpr.as_ref().map(Number::as_str), Some("9"));
    }
}
