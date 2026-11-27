//! 감사용역체결현황 (`adtServcCnclsSttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "adtServcCnclsSttus.json";

/// One row of 감사용역체결현황: audit service contracts.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct AuditContract {
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
    /// 내용
    #[serde(default)]
    pub cn: String,
    /// 보수: ① 2020년 7월 5일까지 사용됨
    #[serde(default, deserialize_with = "decode::number")]
    pub mendng: Option<Number>,
    /// 총소요시간: ① 2020년 7월 5일까지 사용됨
    #[serde(default, deserialize_with = "decode::number")]
    pub tot_reqre_time: Option<Number>,
    /// 감사계약내역(보수): ② 2020년 7월 6일부터 추가됨
    #[serde(default, deserialize_with = "decode::number")]
    pub adt_cntrct_dtls_mendng: Option<Number>,
    /// 감사계약내역(시간): ② 2020년 7월 6일부터 추가됨
    #[serde(default, deserialize_with = "decode::number")]
    pub adt_cntrct_dtls_time: Option<Number>,
    /// 실제수행내역(보수): ② 2020년 7월 6일부터 추가됨
    #[serde(default, deserialize_with = "decode::number")]
    pub real_exc_dtls_mendng: Option<Number>,
    /// 실제수행내역(시간): ② 2020년 7월 6일부터 추가됨
    #[serde(default, deserialize_with = "decode::number")]
    pub real_exc_dtls_time: Option<Number>,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 감사용역체결현황 (`adtServcCnclsSttus.json`): audit service contracts.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn adt_servc_cncls_sttus(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<AuditContract>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/adtServcCnclsSttus.json");
        let rows: Vec<AuditContract> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(
            rows[0].adt_cntrct_dtls_mendng.as_ref().map(Number::as_str),
            Some("8,100")
        );
    }
}
