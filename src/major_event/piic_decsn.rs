//! 유상증자 결정 (`piicDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "piicDecsn.json";

/// One 유상증자 결정 report: decisions on paid-in capital increases (rights
/// offerings), returned by [`Client::piic_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct PaidInCapitalIncrease {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 신주의 종류와 수(보통주식 (주))
    #[serde(default, deserialize_with = "decode::number")]
    pub nstk_ostk_cnt: Option<Number>,
    /// 신주의 종류와 수(기타주식 (주))
    #[serde(default, deserialize_with = "decode::number")]
    pub nstk_estk_cnt: Option<Number>,
    /// 1주당 액면가액 (원)
    #[serde(default, deserialize_with = "decode::number")]
    pub fv_ps: Option<Number>,
    /// 증자전 발행주식총수 (주)(보통주식 (주))
    #[serde(default, deserialize_with = "decode::number")]
    pub bfic_tisstk_ostk: Option<Number>,
    /// 증자전 발행주식총수 (주)(기타주식 (주))
    #[serde(default, deserialize_with = "decode::number")]
    pub bfic_tisstk_estk: Option<Number>,
    /// 자금조달의 목적(시설자금 (원))
    #[serde(default, deserialize_with = "decode::number")]
    pub fdpp_fclt: Option<Number>,
    /// 자금조달의 목적(영업양수자금 (원)): ① 2019년 12월 9일부터 추가됨
    #[serde(default, deserialize_with = "decode::number")]
    pub fdpp_bsninh: Option<Number>,
    /// 자금조달의 목적(운영자금 (원))
    #[serde(default, deserialize_with = "decode::number")]
    pub fdpp_op: Option<Number>,
    /// 자금조달의 목적(채무상환자금 (원)): ① 2019년 12월 9일부터 추가됨
    #[serde(default, deserialize_with = "decode::number")]
    pub fdpp_dtrp: Option<Number>,
    /// 자금조달의 목적(타법인 증권 취득자금 (원))
    #[serde(default, deserialize_with = "decode::number")]
    pub fdpp_ocsa: Option<Number>,
    /// 자금조달의 목적(기타자금 (원))
    #[serde(default, deserialize_with = "decode::number")]
    pub fdpp_etc: Option<Number>,
    /// 증자방식
    #[serde(default)]
    pub ic_mthn: String,
    /// 공매도 해당여부
    #[serde(default)]
    pub ssl_at: String,
    /// 공매도 시작일
    #[serde(default)]
    pub ssl_bgd: String,
    /// 공매도 종료일
    #[serde(default)]
    pub ssl_edd: String,
}

impl Client {
    /// 유상증자 결정 (`piicDecsn.json`): decisions on paid-in capital increases
    /// (rights offerings) filed in the date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn piic_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<PaidInCapitalIncrease>, Error> {
        self.date_range_rows(ENDPOINT, request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_matches_the_guide() {
        crate::guide_check::assert_documented(ENDPOINT, &crate::date_range::example().query());
    }

    #[test]
    fn decodes_fixture() {
        let body = include_bytes!("../../tests/fixtures/major_event/piicDecsn.json");
        let rows: Vec<PaidInCapitalIncrease> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20190221000921");
        assert_eq!(
            rows[0].nstk_ostk_cnt.as_ref().map(Number::as_str),
            Some("376,265")
        );
    }
}
