//! 회사채 미상환 잔액 (`cprndNrdmpBlce.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "cprndNrdmpBlce.json";

/// One row of 회사채 미상환 잔액: outstanding corporate bonds by remaining maturity.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct CorporateBondBalance {
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
    /// 1년 이하
    #[serde(default, deserialize_with = "decode::number")]
    pub yy1_below: Option<Number>,
    /// 1년초과 2년이하
    #[serde(default, deserialize_with = "decode::number")]
    pub yy1_excess_yy2_below: Option<Number>,
    /// 2년초과 3년이하
    #[serde(default, deserialize_with = "decode::number")]
    pub yy2_excess_yy3_below: Option<Number>,
    /// 3년초과 4년이하
    #[serde(default, deserialize_with = "decode::number")]
    pub yy3_excess_yy4_below: Option<Number>,
    /// 4년초과 5년이하
    #[serde(default, deserialize_with = "decode::number")]
    pub yy4_excess_yy5_below: Option<Number>,
    /// 5년초과 10년이하
    #[serde(default, deserialize_with = "decode::number")]
    pub yy5_excess_yy10_below: Option<Number>,
    /// 10년초과
    #[serde(default, deserialize_with = "decode::number")]
    pub yy10_excess: Option<Number>,
    /// 합계
    #[serde(default, deserialize_with = "decode::number")]
    pub sm: Option<Number>,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 회사채 미상환 잔액 (`cprndNrdmpBlce.json`): outstanding corporate bonds by
    /// remaining maturity.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn cprnd_nrdmp_blce(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<CorporateBondBalance>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/cprndNrdmpBlce.json");
        let rows: Vec<CorporateBondBalance> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(
            rows[0].yy1_below.as_ref().map(Number::as_str),
            Some("7,174,000,000")
        );
    }
}
