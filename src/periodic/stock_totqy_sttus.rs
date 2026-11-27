//! 주식의 총수 현황 (`stockTotqySttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "stockTotqySttus.json";

/// One row of 주식의 총수 현황: total number of shares.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct ShareCount {
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
    /// 발행할 주식의 총수: Ⅰ. 발행할 주식의 총수
    #[serde(default, deserialize_with = "decode::number")]
    pub isu_stock_totqy: Option<Number>,
    /// 현재까지 발행한 주식의 총수: Ⅱ. 현재까지 발행한 주식의 총수
    #[serde(default, deserialize_with = "decode::number")]
    pub now_to_isu_stock_totqy: Option<Number>,
    /// 현재까지 감소한 주식의 총수: Ⅲ. 현재까지 감소한 주식의 총수
    #[serde(default, deserialize_with = "decode::number")]
    pub now_to_dcrs_stock_totqy: Option<Number>,
    /// 감자: Ⅲ. 현재까지 감소한 주식의 총수(1. 감자)
    #[serde(default, deserialize_with = "decode::number")]
    pub redc: Option<Number>,
    /// 이익소각: Ⅲ. 현재까지 감소한 주식의 총수(2. 이익소각)
    #[serde(default, deserialize_with = "decode::number")]
    pub profit_incnr: Option<Number>,
    /// 상환주식의 상환: Ⅲ. 현재까지 감소한 주식의 총수(3. 상환주식의 상환)
    #[serde(default, deserialize_with = "decode::number")]
    pub rdmstk_repy: Option<Number>,
    /// 기타: Ⅲ. 현재까지 감소한 주식의 총수(4. 기타)
    #[serde(default, deserialize_with = "decode::number")]
    pub etc: Option<Number>,
    /// 발행주식의 총수: Ⅳ. 발행주식의 총수 (Ⅱ-Ⅲ)
    #[serde(default, deserialize_with = "decode::number")]
    pub istc_totqy: Option<Number>,
    /// 자기주식수: Ⅴ. 자기주식수
    #[serde(default, deserialize_with = "decode::number")]
    pub tesstk_co: Option<Number>,
    /// 유통주식수: Ⅵ. 유통주식수 (Ⅳ-Ⅴ)
    #[serde(default, deserialize_with = "decode::number")]
    pub distb_stock_co: Option<Number>,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 주식의 총수 현황 (`stockTotqySttus.json`): total number of shares.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn stock_totqy_sttus(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<ShareCount>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/stockTotqySttus.json");
        let rows: Vec<ShareCount> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(
            rows[0].isu_stock_totqy.as_ref().map(Number::as_str),
            Some("20,000,000,000")
        );
    }
}
