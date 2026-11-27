//! 직원 현황 (`empSttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "empSttus.json";

/// One row of 직원 현황: employees.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct Employees {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 법인명
    pub corp_name: String,
    /// 사 업부문
    #[serde(default)]
    pub fo_bbm: String,
    /// 성별
    #[serde(default)]
    pub sexdstn: String,
    /// 개정 전 직원 수 정규직
    #[serde(default)]
    pub reform_bfe_emp_co_rgllbr: String,
    /// 개정 전 직원 수 계약직
    #[serde(default)]
    pub reform_bfe_emp_co_cnttk: String,
    /// 개정 전 직원 수 기타
    #[serde(default)]
    pub reform_bfe_emp_co_etc: String,
    /// 정규직 수
    #[serde(default, deserialize_with = "decode::number")]
    pub rgllbr_co: Option<Number>,
    /// 정규직 단시간 근로자 수
    #[serde(default, deserialize_with = "decode::number")]
    pub rgllbr_abacpt_labrr_co: Option<Number>,
    /// 계약직 수
    #[serde(default, deserialize_with = "decode::number")]
    pub cnttk_co: Option<Number>,
    /// 계약직 단시간 근로자 수
    #[serde(default, deserialize_with = "decode::number")]
    pub cnttk_abacpt_labrr_co: Option<Number>,
    /// 합계
    #[serde(default, deserialize_with = "decode::number")]
    pub sm: Option<Number>,
    /// 평균 근속 연수
    #[serde(default, deserialize_with = "decode::number")]
    pub avrg_cnwk_sdytrn: Option<Number>,
    /// 연간 급여 총액
    #[serde(default, deserialize_with = "decode::number")]
    pub fyer_salary_totamt: Option<Number>,
    /// 1인평균 급여 액
    #[serde(default, deserialize_with = "decode::number")]
    pub jan_salary_am: Option<Number>,
    /// 비고
    #[serde(default)]
    pub rm: String,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 직원 현황 (`empSttus.json`): employees.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn emp_sttus(&self, request: &ReportRequest) -> Result<Vec<Employees>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/empSttus.json");
        let rows: Vec<Employees> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(
            rows[0].rgllbr_co.as_ref().map(Number::as_str),
            Some("37,787")
        );
    }
}
