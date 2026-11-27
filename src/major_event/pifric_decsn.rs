//! 유무상증자 결정 (`pifricDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "pifricDecsn.json";

/// One 유무상증자 결정 report: decisions on combined paid-in capital increases and
/// bonus issues, returned by [`Client::pifric_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct PaidInAndBonusIssue {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 유상증자(신주의 종류와 수(보통주식 (주)))
    #[serde(default, deserialize_with = "decode::number")]
    pub piic_nstk_ostk_cnt: Option<Number>,
    /// 유상증자(신주의 종류와 수(기타주식 (주)))
    #[serde(default, deserialize_with = "decode::number")]
    pub piic_nstk_estk_cnt: Option<Number>,
    /// 유상증자(1주당 액면가액 (원))
    #[serde(default, deserialize_with = "decode::number")]
    pub piic_fv_ps: Option<Number>,
    /// 유상증자(증자전 발행주식총수 (주)(보통주식 (주)))
    #[serde(default, deserialize_with = "decode::number")]
    pub piic_bfic_tisstk_ostk: Option<Number>,
    /// 유상증자(증자전 발행주식총수 (주)(기타주식 (주)))
    #[serde(default, deserialize_with = "decode::number")]
    pub piic_bfic_tisstk_estk: Option<Number>,
    /// 유상증자(자금조달의 목적(시설자금 (원)))
    #[serde(default, deserialize_with = "decode::number")]
    pub piic_fdpp_fclt: Option<Number>,
    /// 유상증자(자금조달의 목적(영업양수자금 (원))): ① 2019년 12월 9일부터 추가됨
    #[serde(default, deserialize_with = "decode::number")]
    pub piic_fdpp_bsninh: Option<Number>,
    /// 유상증자(자금조달의 목적(운영자금 (원)))
    #[serde(default, deserialize_with = "decode::number")]
    pub piic_fdpp_op: Option<Number>,
    /// 유상증자(자금조달의 목적(채무상환자금 (원))): ① 2019년 12월 9일부터 추가됨
    #[serde(default, deserialize_with = "decode::number")]
    pub piic_fdpp_dtrp: Option<Number>,
    /// 유상증자(자금조달의 목적(타법인 증권 취득자금 (원)))
    #[serde(default, deserialize_with = "decode::number")]
    pub piic_fdpp_ocsa: Option<Number>,
    /// 유상증자(자금조달의 목적(기타자금 (원)))
    #[serde(default, deserialize_with = "decode::number")]
    pub piic_fdpp_etc: Option<Number>,
    /// 유상증자(증자방식)
    #[serde(default)]
    pub piic_ic_mthn: String,
    /// 무상증자(신주의 종류와 수(보통주식 (주)))
    #[serde(default, deserialize_with = "decode::number")]
    pub fric_nstk_ostk_cnt: Option<Number>,
    /// 무상증자(신주의 종류와 수(기타주식 (주)))
    #[serde(default, deserialize_with = "decode::number")]
    pub fric_nstk_estk_cnt: Option<Number>,
    /// 무상증자(1주당 액면가액 (원))
    #[serde(default, deserialize_with = "decode::number")]
    pub fric_fv_ps: Option<Number>,
    /// 무상증자(증자전 발행주식총수(보통주식 (주)))
    #[serde(default, deserialize_with = "decode::number")]
    pub fric_bfic_tisstk_ostk: Option<Number>,
    /// 무상증자(증자전 발행주식총수(기타주식 (주)))
    #[serde(default, deserialize_with = "decode::number")]
    pub fric_bfic_tisstk_estk: Option<Number>,
    /// 무상증자(신주배정기준일)
    #[serde(default)]
    pub fric_nstk_asstd: String,
    /// 무상증자(1주당 신주배정 주식수(보통주식 (주)))
    #[serde(default, deserialize_with = "decode::number")]
    pub fric_nstk_ascnt_ps_ostk: Option<Number>,
    /// 무상증자(1주당 신주배정 주식수(기타주식 (주)))
    #[serde(default, deserialize_with = "decode::number")]
    pub fric_nstk_ascnt_ps_estk: Option<Number>,
    /// 무상증자(신주의 배당기산일)
    #[serde(default)]
    pub fric_nstk_dividrk: String,
    /// 무상증자(신주권교부예정일)
    #[serde(default)]
    pub fric_nstk_dlprd: String,
    /// 무상증자(신주의 상장 예정일)
    #[serde(default)]
    pub fric_nstk_lstprd: String,
    /// 무상증자(이사회결의일(결정일))
    #[serde(default)]
    pub fric_bddd: String,
    /// 무상증자(독립(사외)이사 참석여부(참석(명)))
    #[serde(default, deserialize_with = "decode::number")]
    pub fric_od_a_at_t: Option<Number>,
    /// 무상증자(독립(사외)이사 참석여부(불참(명)))
    #[serde(default, deserialize_with = "decode::number")]
    pub fric_od_a_at_b: Option<Number>,
    /// 무상증자(감사(감사위원)참석 여부)
    #[serde(default)]
    pub fric_adt_a_atn: String,
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
    /// 유무상증자 결정 (`pifricDecsn.json`): decisions on combined paid-in capital
    /// increases and bonus issues filed in the date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn pifric_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<PaidInAndBonusIssue>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/pifricDecsn.json");
        let rows: Vec<PaidInAndBonusIssue> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20190801000415");
        assert_eq!(
            rows[0].piic_nstk_ostk_cnt.as_ref().map(Number::as_str),
            Some("1,100,000")
        );
    }
}
