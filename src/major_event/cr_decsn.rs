//! 감자 결정 (`crDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "crDecsn.json";

/// One 감자 결정 report: decisions on capital reductions, returned by
/// [`Client::cr_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct CapitalReduction {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 감자주식의 종류와 수(보통주식 (주))
    #[serde(default, deserialize_with = "decode::number")]
    pub crstk_ostk_cnt: Option<Number>,
    /// 감자주식의 종류와 수(기타주식 (주))
    #[serde(default, deserialize_with = "decode::number")]
    pub crstk_estk_cnt: Option<Number>,
    /// 1주당 액면가액 (원)
    #[serde(default, deserialize_with = "decode::number")]
    pub fv_ps: Option<Number>,
    /// 감자전후 자본금(감자전 (원))
    #[serde(default, deserialize_with = "decode::number")]
    pub bfcr_cpt: Option<Number>,
    /// 감자전후 자본금(감자후 (원))
    #[serde(default, deserialize_with = "decode::number")]
    pub atcr_cpt: Option<Number>,
    /// 감자전후 발행주식수(보통주식 (주)(감자전 (원)))
    #[serde(default, deserialize_with = "decode::number")]
    pub bfcr_tisstk_ostk: Option<Number>,
    /// 감자전후 발행주식수(보통주식 (주)(감자후 (원)))
    #[serde(default, deserialize_with = "decode::number")]
    pub atcr_tisstk_ostk: Option<Number>,
    /// 감자전후 발행주식수(기타주식 (주)(감자전 (원)))
    #[serde(default, deserialize_with = "decode::number")]
    pub bfcr_tisstk_estk: Option<Number>,
    /// 감자전후 발행주식수(기타주식 (주)(감자후 (원)))
    #[serde(default, deserialize_with = "decode::number")]
    pub atcr_tisstk_estk: Option<Number>,
    /// 감자비율(보통주식 (%))
    #[serde(default, deserialize_with = "decode::number")]
    pub cr_rt_ostk: Option<Number>,
    /// 감자비율(기타주식 (%))
    #[serde(default, deserialize_with = "decode::number")]
    pub cr_rt_estk: Option<Number>,
    /// 감자기준일
    #[serde(default)]
    pub cr_std: String,
    /// 감자방법
    #[serde(default)]
    pub cr_mth: String,
    /// 감자사유
    #[serde(default)]
    pub cr_rs: String,
    /// 감자일정(주주총회 예정일)
    #[serde(default)]
    pub crsc_gmtsck_prd: String,
    /// 감자일정(명의개서정지기간)
    #[serde(default)]
    pub crsc_trnmsppd: String,
    /// 감자일정(구주권 제출기간): ① 2019년 12월 8일까지 사용됨
    #[serde(default)]
    pub crsc_osprpd: String,
    /// 감자일정(매매거래 정지예정기간): ① 2019년 12월 8일까지 사용됨
    #[serde(default)]
    pub crsc_trspprpd: String,
    /// 감자일정(구주권 제출기간(시작일)): ② 2019년 12월 9일부터 추가됨
    #[serde(default)]
    pub crsc_osprpd_bgd: String,
    /// 감자일정(구주권 제출기간(종료일)): ② 2019년 12월 9일부터 추가됨
    #[serde(default)]
    pub crsc_osprpd_edd: String,
    /// 감자일정(매매거래 정지예정기간(시작일)): ② 2019년 12월 9일부터 추가됨
    #[serde(default)]
    pub crsc_trspprpd_bgd: String,
    /// 감자일정(매매거래 정지예정기간(종료일)): ② 2019년 12월 9일부터 추가됨
    #[serde(default)]
    pub crsc_trspprpd_edd: String,
    /// 감자일정(신주권교부예정일)
    #[serde(default)]
    pub crsc_nstkdlprd: String,
    /// 감자일정(신주상장예정일)
    #[serde(default)]
    pub crsc_nstklstprd: String,
    /// 채권자 이의제출기간(시작일)
    #[serde(default)]
    pub cdobprpd_bgd: String,
    /// 채권자 이의제출기간(종료일)
    #[serde(default)]
    pub cdobprpd_edd: String,
    /// 구주권제출 및 신주권교부장소
    #[serde(default)]
    pub ospr_nstkdl_pl: String,
    /// 이사회결의일(결정일)
    #[serde(default)]
    pub bddd: String,
    /// 독립(사외)이사 참석여부(참석(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_t: Option<Number>,
    /// 독립(사외)이사 참석여부(불참(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_b: Option<Number>,
    /// 감사(감사위원) 참석여부
    #[serde(default)]
    pub adt_a_atn: String,
    /// 공정거래위원회 신고대상 여부
    #[serde(default)]
    pub ftc_stt_atn: String,
}

impl Client {
    /// 감자 결정 (`crDecsn.json`): decisions on capital reductions filed in the
    /// date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn cr_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<CapitalReduction>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/crDecsn.json");
        let rows: Vec<CapitalReduction> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20190513000176");
        assert_eq!(rows[0].fv_ps.as_ref().map(Number::as_str), Some("5,000"));
    }
}
