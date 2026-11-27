//! 타법인 출자현황 (`otrCprInvstmntSttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "otrCprInvstmntSttus.json";

/// One row of 타법인 출자현황: investments in other companies.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct Investment {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 법인명
    pub corp_name: String,
    /// 법인명: the company invested in.
    #[serde(default)]
    pub inv_prm: String,
    /// 최초 취득 일자
    #[serde(default)]
    pub frst_acqs_de: String,
    /// 출자 목적
    #[serde(default)]
    pub invstmnt_purps: String,
    /// 최초 취득 금액
    #[serde(default, deserialize_with = "decode::number")]
    pub frst_acqs_amount: Option<Number>,
    /// 기초 잔액 수량
    #[serde(default, deserialize_with = "decode::number")]
    pub bsis_blce_qy: Option<Number>,
    /// 기초 잔액 지분 율
    #[serde(default, deserialize_with = "decode::number")]
    pub bsis_blce_qota_rt: Option<Number>,
    /// 기초 잔액 장부 가액
    #[serde(default, deserialize_with = "decode::number")]
    pub bsis_blce_acntbk_amount: Option<Number>,
    /// 증가 감소 취득 처분 수량
    #[serde(default, deserialize_with = "decode::number")]
    pub incrs_dcrs_acqs_dsps_qy: Option<Number>,
    /// 증가 감소 취득 처분 금액
    #[serde(default, deserialize_with = "decode::number")]
    pub incrs_dcrs_acqs_dsps_amount: Option<Number>,
    /// 증가 감소 평가 손액
    #[serde(default, deserialize_with = "decode::number")]
    pub incrs_dcrs_evl_lstmn: Option<Number>,
    /// 기말 잔액 수량
    #[serde(default, deserialize_with = "decode::number")]
    pub trmend_blce_qy: Option<Number>,
    /// 기말 잔액 지분 율
    #[serde(default, deserialize_with = "decode::number")]
    pub trmend_blce_qota_rt: Option<Number>,
    /// 기말 잔액 장부 가액
    #[serde(default, deserialize_with = "decode::number")]
    pub trmend_blce_acntbk_amount: Option<Number>,
    /// 최근 사업 연도 재무 현황 총 자산
    #[serde(default, deserialize_with = "decode::number")]
    pub recent_bsns_year_fnnr_sttus_tot_assets: Option<Number>,
    /// 최근 사업 연도 재무 현황 당기 순이익
    #[serde(default, deserialize_with = "decode::number")]
    pub recent_bsns_year_fnnr_sttus_thstrm_ntpf: Option<Number>,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 타법인 출자현황 (`otrCprInvstmntSttus.json`): investments in other companies.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn otr_cpr_invstmnt_sttus(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<Investment>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/otrCprInvstmntSttus.json");
        let rows: Vec<Investment> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(
            rows[0].frst_acqs_amount.as_ref().map(Number::as_str),
            Some("250,000,000")
        );
    }
}
