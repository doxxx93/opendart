//! 최대주주 변동현황 (`hyslrChgSttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "hyslrChgSttus.json";

/// One row of 최대주주 변동현황: changes of the largest shareholder.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct LargestShareholderChange {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 법인명
    pub corp_name: String,
    /// 변동 일
    #[serde(default)]
    pub change_on: String,
    /// 최대 주주 명
    #[serde(default)]
    pub mxmm_shrholdr_nm: String,
    /// 소유 주식 수
    #[serde(default, deserialize_with = "decode::number")]
    pub posesn_stock_co: Option<Number>,
    /// 지분 율
    #[serde(default, deserialize_with = "decode::number")]
    pub qota_rt: Option<Number>,
    /// 변동 원인
    #[serde(default)]
    pub change_cause: String,
    /// 비고
    #[serde(default)]
    pub rm: String,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 최대주주 변동현황 (`hyslrChgSttus.json`): changes of the largest shareholder.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn hyslr_chg_sttus(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<LargestShareholderChange>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/hyslrChgSttus.json");
        let rows: Vec<LargestShareholderChange> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20191227000084");
        assert_eq!(
            rows[0].posesn_stock_co.as_ref().map(Number::as_str),
            Some("3,279,279")
        );
    }
}
