//! 기업어음증권 미상환 잔액 (`entrprsBilScritsNrdmpBlce.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "entrprsBilScritsNrdmpBlce.json";

/// One row of 기업어음증권 미상환 잔액: outstanding commercial paper by remaining
/// maturity.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct CommercialPaperBalance {
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
    /// 1년초과 2년이하
    #[serde(default, deserialize_with = "decode::number")]
    pub yy1_excess_yy2_below: Option<Number>,
    /// 2년초과 3년이하
    #[serde(default, deserialize_with = "decode::number")]
    pub yy2_excess_yy3_below: Option<Number>,
    /// 3년 초과
    #[serde(default, deserialize_with = "decode::number")]
    pub yy3_excess: Option<Number>,
    /// 합계
    #[serde(default, deserialize_with = "decode::number")]
    pub sm: Option<Number>,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 기업어음증권 미상환 잔액 (`entrprsBilScritsNrdmpBlce.json`): outstanding commercial
    /// paper by remaining maturity.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn entrprs_bil_scrits_nrdmp_blce(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<CommercialPaperBalance>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/entrprsBilScritsNrdmpBlce.json");
        let rows: Vec<CommercialPaperBalance> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].rcept_no, "20260310002820");
    }
}
