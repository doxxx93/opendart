//! 단일회사 주요 재무지표 (`fnlttSinglIndx.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::client::Query;
use crate::{Client, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "fnlttSinglIndx.json";

/// Indicator group (`idx_cl_code`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IndicatorClass {
    /// `M210000`: 수익성지표 (profitability).
    Profitability,
    /// `M220000`: 안정성지표 (stability).
    Stability,
    /// `M230000`: 성장성지표 (growth).
    Growth,
    /// `M240000`: 활동성지표 (activity).
    Activity,
}

impl IndicatorClass {
    /// The raw code, e.g. `"M210000"`.
    pub fn code(self) -> &'static str {
        match self {
            IndicatorClass::Profitability => "M210000",
            IndicatorClass::Stability => "M220000",
            IndicatorClass::Growth => "M230000",
            IndicatorClass::Activity => "M240000",
        }
    }
}

/// One financial indicator, returned by [`Client::fnltt_singl_indx`] and
/// [`Client::fnltt_cmpny_indx`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct Indicator {
    /// 보고서 코드, e.g. `11011`.
    #[serde(default)]
    pub reprt_code: String,
    /// 사업연도
    #[serde(default)]
    pub bsns_year: String,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 종목코드
    #[serde(default, deserialize_with = "decode::opt_string")]
    pub stock_code: Option<String>,
    /// 결산기준일
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
    /// 지표분류코드, e.g. `M210000`.
    #[serde(default)]
    pub idx_cl_code: String,
    /// 지표분류명, e.g. `수익성지표`.
    #[serde(default)]
    pub idx_cl_nm: String,
    /// 지표코드, e.g. `M211000`.
    #[serde(default)]
    pub idx_code: String,
    /// 지표명, e.g. `영업이익률`.
    #[serde(default)]
    pub idx_nm: String,
    /// 지표값; `None` when the indicator does not apply.
    #[serde(default, deserialize_with = "decode::number")]
    pub idx_val: Option<Number>,
}

impl Client {
    /// 단일회사 주요 재무지표 (`fnlttSinglIndx.json`): one group of
    /// financial indicators (ratios) for one company's report. Data starts
    /// with the third quarter of 2023.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn fnltt_singl_indx(
        &self,
        request: &ReportRequest,
        idx_cl_code: IndicatorClass,
    ) -> Result<Vec<Indicator>, Error> {
        self.rows(ENDPOINT, &query(request.query(), idx_cl_code))
            .await
    }
}

pub(super) fn query(mut query: Query, idx_cl_code: IndicatorClass) -> Query {
    query.push(("idx_cl_code", idx_cl_code.code().to_owned()));
    query
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn request_matches_the_guide() {
        let request = ReportRequest::new("00126380", 2025, crate::ReportCode::Annual);
        crate::guide_check::assert_documented(
            ENDPOINT,
            &query(request.query(), IndicatorClass::Growth),
        );
    }

    #[test]
    fn decodes_fixture() {
        let body = include_bytes!("../../tests/fixtures/financial/fnlttSinglIndx.json");
        let rows: Vec<Indicator> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 5);
        assert_eq!(rows[0].idx_cl_code, "M210000");
        assert_eq!(rows[0].stlm_dt, Some(date(2025, 12, 31)));
        assert!(rows.iter().any(|row| row.idx_val.is_none()));
        assert!(
            rows.iter()
                .filter_map(|row| row.idx_val.as_ref())
                .all(|value| value.parse::<f64>().is_ok())
        );
    }
}
