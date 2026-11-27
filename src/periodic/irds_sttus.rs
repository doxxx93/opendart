//! 증자(감자) 현황 (`irdsSttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "irdsSttus.json";

/// One row of 증자(감자) 현황: changes in paid-in capital: issuance and reduction of
/// shares.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct CapitalChange {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 법인명
    pub corp_name: String,
    /// 주식발행 감소일자
    #[serde(default)]
    pub isu_dcrs_de: String,
    /// 발행 감소 형태
    #[serde(default)]
    pub isu_dcrs_stle: String,
    /// 발행 감소 주식 종류
    #[serde(default)]
    pub isu_dcrs_stock_knd: String,
    /// 발행 감소 수량
    #[serde(default, deserialize_with = "decode::number")]
    pub isu_dcrs_qy: Option<Number>,
    /// 발행 감소 주당 액면 가액
    #[serde(default, deserialize_with = "decode::number")]
    pub isu_dcrs_mstvdv_fval_amount: Option<Number>,
    /// 발행 감소 주당 가액
    #[serde(default, deserialize_with = "decode::number")]
    pub isu_dcrs_mstvdv_amount: Option<Number>,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 증자(감자) 현황 (`irdsSttus.json`): changes in paid-in capital: issuance and
    /// reduction of shares.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn irds_sttus(&self, request: &ReportRequest) -> Result<Vec<CapitalChange>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/irdsSttus.json");
        let rows: Vec<CapitalChange> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].rcept_no, "20191227000084");
        assert_eq!(
            rows[0].isu_dcrs_qy.as_ref().map(Number::as_str),
            Some("173,949")
        );
    }
}
