//! 이사·감사 전체의 보수현황(주주총회 승인금액) (`drctrAdtAllMendngSttusGmtsckConfmAmount.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "drctrAdtAllMendngSttusGmtsckConfmAmount.json";

/// One row of 이사·감사 전체의 보수현황(주주총회 승인금액): pay limit for directors and auditors
/// approved by the shareholders' meeting.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct ApprovedDirectorPay {
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
    /// 주주총회 승인금액
    #[serde(default, deserialize_with = "decode::number")]
    pub gmtsck_confm_amount: Option<Number>,
    /// 비고
    #[serde(default)]
    pub rm: String,
    /// 사업연도
    #[serde(default)]
    pub fscl_year: String,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 이사·감사 전체의 보수현황(주주총회 승인금액)
    /// (`drctrAdtAllMendngSttusGmtsckConfmAmount.json`): pay limit for
    /// directors and auditors approved by the shareholders' meeting.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn drctr_adt_all_mendng_sttus_gmtsck_confm_amount(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<ApprovedDirectorPay>, Error> {
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
        let body = include_bytes!(
            "../../tests/fixtures/periodic/drctrAdtAllMendngSttusGmtsckConfmAmount.json"
        );
        let rows: Vec<ApprovedDirectorPay> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(rows[0].nmpr.as_ref().map(Number::as_str), Some("4"));
    }
}
