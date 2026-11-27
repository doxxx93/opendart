//! 무상증자 결정 (`fricDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "fricDecsn.json";

/// One 무상증자 결정 report: decisions on bonus issues, returned by
/// [`Client::fric_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct BonusIssue {
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
    /// 신주배정기준일
    #[serde(default)]
    pub nstk_asstd: String,
    /// 1주당 신주배정 주식수(보통주식 (주))
    #[serde(default, deserialize_with = "decode::number")]
    pub nstk_ascnt_ps_ostk: Option<Number>,
    /// 1주당 신주배정 주식수(기타주식 (주))
    #[serde(default, deserialize_with = "decode::number")]
    pub nstk_ascnt_ps_estk: Option<Number>,
    /// 신주의 배당기산일
    #[serde(default)]
    pub nstk_dividrk: String,
    /// 신주권교부예정일
    #[serde(default)]
    pub nstk_dlprd: String,
    /// 신주의 상장 예정일
    #[serde(default)]
    pub nstk_lstprd: String,
    /// 이사회결의일(결정일)
    #[serde(default)]
    pub bddd: String,
    /// 독립(사외)이사 참석여부(참석(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_t: Option<Number>,
    /// 독립(사외)이사 참석여부(불참(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_b: Option<Number>,
    /// 감사(감사위원)참석 여부
    #[serde(default)]
    pub adt_a_atn: String,
}

impl Client {
    /// 무상증자 결정 (`fricDecsn.json`): decisions on bonus issues filed in the date
    /// range.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn fric_decsn(&self, request: &DateRangeRequest) -> Result<Vec<BonusIssue>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/fricDecsn.json");
        let rows: Vec<BonusIssue> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20190228000002");
        assert_eq!(
            rows[0].nstk_ostk_cnt.as_ref().map(Number::as_str),
            Some("4,259,883")
        );
    }
}
