//! 임원 현황 (`exctvSttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, ReportRequest, decode};

const ENDPOINT: &str = "exctvSttus.json";

/// One row of 임원 현황: executives.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct Executive {
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
    /// 성별
    #[serde(default)]
    pub sexdstn: String,
    /// 출생 년월
    #[serde(default)]
    pub birth_ym: String,
    /// 직위
    #[serde(default)]
    pub ofcps: String,
    /// 등기 임원 여부
    #[serde(default)]
    pub rgist_exctv_at: String,
    /// 상근 여부
    #[serde(default)]
    pub fte_at: String,
    /// 담당 업무
    #[serde(default)]
    pub chrg_job: String,
    /// 주요 경력
    #[serde(default)]
    pub main_career: String,
    /// 최대 주주 관계
    #[serde(default)]
    pub mxmm_shrholdr_relate: String,
    /// 재직 기간
    #[serde(default)]
    pub hffc_pd: String,
    /// 임기 만료 일
    #[serde(default)]
    pub tenure_end_on: String,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 임원 현황 (`exctvSttus.json`): executives.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn exctv_sttus(&self, request: &ReportRequest) -> Result<Vec<Executive>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/exctvSttus.json");
        let rows: Vec<Executive> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].rcept_no, "20260310002820");
    }
}
