//! 회계감사인의 명칭 및 감사의견 (`accnutAdtorNmNdAdtOpinion.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, ReportRequest, decode};

const ENDPOINT: &str = "accnutAdtorNmNdAdtOpinion.json";

/// One row of 회계감사인의 명칭 및 감사의견: external auditor and audit opinion.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct AuditOpinion {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 법인명
    pub corp_name: String,
    /// 사업연도
    #[serde(default)]
    pub bsns_year: String,
    /// 감사인
    #[serde(default)]
    pub adtor: String,
    /// 감사의견
    #[serde(default)]
    pub adt_opinion: String,
    /// 감사보고서 특기사항: ① 2019년 12월 8일까지 사용됨
    #[serde(default)]
    pub adt_reprt_spcmnt_matter: String,
    /// 강조사항 등: ② 2019년 12월 9일부터 추가됨
    #[serde(default)]
    pub emphs_matter: String,
    /// 핵심감사사항: ② 2019년 12월 9일부터 추가됨
    #[serde(default)]
    pub core_adt_matter: String,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 회계감사인의 명칭 및 감사의견 (`accnutAdtorNmNdAdtOpinion.json`): external auditor
    /// and audit opinion.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn accnut_adtor_nm_nd_adt_opinion(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<AuditOpinion>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/accnutAdtorNmNdAdtOpinion.json");
        let rows: Vec<AuditOpinion> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].rcept_no, "20260310002820");
    }
}
