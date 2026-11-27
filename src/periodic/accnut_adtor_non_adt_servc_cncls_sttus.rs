//! 회계감사인과의 비감사용역 계약체결 현황 (`accnutAdtorNonAdtServcCnclsSttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "accnutAdtorNonAdtServcCnclsSttus.json";

/// One row of 회계감사인과의 비감사용역 계약체결 현황: non-audit service contracts with the
/// external auditor.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct NonAuditContract {
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
    /// 계약체결일
    #[serde(default)]
    pub cntrct_cncls_de: String,
    /// 용역내용
    #[serde(default)]
    pub servc_cn: String,
    /// 용역수행기간
    #[serde(default)]
    pub servc_exc_pd: String,
    /// 용역보수
    #[serde(default, deserialize_with = "decode::number")]
    pub servc_mendng: Option<Number>,
    /// 비고
    #[serde(default)]
    pub rm: String,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 회계감사인과의 비감사용역 계약체결 현황 (`accnutAdtorNonAdtServcCnclsSttus.json`):
    /// non-audit service contracts with the external auditor.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn accnut_adtor_non_adt_servc_cncls_sttus(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<NonAuditContract>, Error> {
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
        let body =
            include_bytes!("../../tests/fixtures/periodic/accnutAdtorNonAdtServcCnclsSttus.json");
        let rows: Vec<NonAuditContract> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(
            rows[1].servc_mendng.as_ref().map(Number::as_str),
            Some("43")
        );
    }
}
