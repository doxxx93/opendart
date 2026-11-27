//! 단기사채 미상환 잔액 (`srtpdPsndbtNrdmpBlce.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "srtpdPsndbtNrdmpBlce.json";

/// One row of 단기사채 미상환 잔액: outstanding short-term bonds by remaining maturity.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct ShortTermBondBalance {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 법인명
    pub corp_name: String,
    /// 잔여만기
    #[serde(default)]
    pub remndr_exprtn1: String,
    /// 잔여만기
    #[serde(default)]
    pub remndr_exprtn2: String,
    /// 10일 이하
    #[serde(default, deserialize_with = "decode::number")]
    pub de10_below: Option<Number>,
    /// 10일초과 30일이하
    #[serde(default, deserialize_with = "decode::number")]
    pub de10_excess_de30_below: Option<Number>,
    /// 30일초과 90일이하
    #[serde(default, deserialize_with = "decode::number")]
    pub de30_excess_de90_below: Option<Number>,
    /// 90일초과 180일이하
    #[serde(default, deserialize_with = "decode::number")]
    pub de90_excess_de180_below: Option<Number>,
    /// 180일초과 1년이하
    #[serde(default, deserialize_with = "decode::number")]
    pub de180_excess_yy1_below: Option<Number>,
    /// 합계
    #[serde(default, deserialize_with = "decode::number")]
    pub sm: Option<Number>,
    /// 발행 한도
    #[serde(default, deserialize_with = "decode::number")]
    pub isu_lmt: Option<Number>,
    /// 잔여 한도
    #[serde(default, deserialize_with = "decode::number")]
    pub remndr_lmt: Option<Number>,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 단기사채 미상환 잔액 (`srtpdPsndbtNrdmpBlce.json`): outstanding short-term bonds
    /// by remaining maturity.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn srtpd_psndbt_nrdmp_blce(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<ShortTermBondBalance>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/srtpdPsndbtNrdmpBlce.json");
        let rows: Vec<ShortTermBondBalance> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].rcept_no, "20260310002820");
    }
}
