//! 소액주주 현황 (`mrhlSttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "mrhlSttus.json";

/// One row of 소액주주 현황: minority shareholders.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct MinorityShareholding {
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
    /// 주주수
    #[serde(default, deserialize_with = "decode::number")]
    pub shrholdr_co: Option<Number>,
    /// 전체 주주수
    #[serde(default, deserialize_with = "decode::number")]
    pub shrholdr_tot_co: Option<Number>,
    /// 주주 비율
    #[serde(default, deserialize_with = "decode::number")]
    pub shrholdr_rate: Option<Number>,
    /// 보유 주식수
    #[serde(default, deserialize_with = "decode::number")]
    pub hold_stock_co: Option<Number>,
    /// 총발행 주식수
    #[serde(default, deserialize_with = "decode::number")]
    pub stock_tot_co: Option<Number>,
    /// 보유 주식 비율
    #[serde(default, deserialize_with = "decode::number")]
    pub hold_stock_rate: Option<Number>,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 소액주주 현황 (`mrhlSttus.json`): minority shareholders.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn mrhl_sttus(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<MinorityShareholding>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/mrhlSttus.json");
        let rows: Vec<MinorityShareholding> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(
            rows[0].shrholdr_co.as_ref().map(Number::as_str),
            Some("4,195,927")
        );
    }
}
