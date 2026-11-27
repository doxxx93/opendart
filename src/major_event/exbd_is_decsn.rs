//! 교환사채권 발행결정 (`exbdIsDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "exbdIsDecsn.json";

/// One 교환사채권 발행결정 report: decisions to issue exchangeable bonds, returned by
/// [`Client::exbd_is_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct ExchangeableBondIssue {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 사채의 종류(회차)
    #[serde(default)]
    pub bd_tm: String,
    /// 사채의 종류(종류)
    #[serde(default)]
    pub bd_knd: String,
    /// 사채의 권면(전자등록)총액 (원)
    #[serde(default, deserialize_with = "decode::number")]
    pub bd_fta: Option<Number>,
    /// 해외발행(권면(전자등록)총액)
    #[serde(default, deserialize_with = "decode::number")]
    pub ovis_fta: Option<Number>,
    /// 해외발행(권면(전자등록)총액(통화단위))
    #[serde(default)]
    pub ovis_fta_crn: String,
    /// 해외발행(기준환율등)
    #[serde(default, deserialize_with = "decode::number")]
    pub ovis_ster: Option<Number>,
    /// 해외발행(발행지역)
    #[serde(default)]
    pub ovis_isar: String,
    /// 해외발행(해외상장시 시장의 명칭)
    #[serde(default)]
    pub ovis_mktnm: String,
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
    /// 사채의 이율(표면이자율 (%))
    #[serde(default, deserialize_with = "decode::number")]
    pub bd_intr_ex: Option<Number>,
    /// 사채의 이율(만기이자율 (%))
    #[serde(default, deserialize_with = "decode::number")]
    pub bd_intr_sf: Option<Number>,
    /// 사채만기일
    #[serde(default)]
    pub bd_mtd: String,
    /// 사채발행방법
    #[serde(default)]
    pub bdis_mthn: String,
    /// 교환에 관한 사항(교환비율 (%))
    #[serde(default, deserialize_with = "decode::number")]
    pub ex_rt: Option<Number>,
    /// 교환에 관한 사항(교환가액 (원/주))
    #[serde(default, deserialize_with = "decode::number")]
    pub ex_prc: Option<Number>,
    /// 교환에 관한 사항(교환가액 결정방법)
    #[serde(default)]
    pub ex_prc_dmth: String,
    /// 교환에 관한 사항(교환대상(종류))
    #[serde(default)]
    pub extg: String,
    /// 교환에 관한 사항(교환대상(주식수))
    #[serde(default, deserialize_with = "decode::number")]
    pub extg_stkcnt: Option<Number>,
    /// 교환에 관한 사항(교환대상(주식총수 대비 비율(%)))
    #[serde(default, deserialize_with = "decode::number")]
    pub extg_tisstk_vs: Option<Number>,
    /// 교환에 관한 사항(교환청구기간(시작일))
    #[serde(default)]
    pub exrqpd_bgd: String,
    /// 교환에 관한 사항(교환청구기간(종료일))
    #[serde(default)]
    pub exrqpd_edd: String,
    /// 청약일
    #[serde(default)]
    pub sbd: String,
    /// 납입일
    #[serde(default)]
    pub pymd: String,
    /// 대표주관회사
    #[serde(default)]
    pub rpmcmp: String,
    /// 보증기관
    #[serde(default)]
    pub grint: String,
    /// 이사회결의일(결정일)
    #[serde(default)]
    pub bddd: String,
    /// 독립(사외)이사 참석여부(참석 (명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_t: Option<Number>,
    /// 독립(사외)이사 참석여부(불참 (명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_b: Option<Number>,
    /// 감사(감사위원) 참석여부
    #[serde(default)]
    pub adt_a_atn: String,
    /// 증권신고서 제출대상 여부
    #[serde(default)]
    pub rs_sm_atn: String,
    /// 제출을 면제받은 경우 그 사유
    #[serde(default)]
    pub ex_sm_r: String,
    /// 당해 사채의 해외발행과 연계된 대차거래 내역
    #[serde(default)]
    pub ovis_ltdtl: String,
    /// 공정거래위원회 신고대상 여부
    #[serde(default)]
    pub ftc_stt_atn: String,
}

impl Client {
    /// 교환사채권 발행결정 (`exbdIsDecsn.json`): decisions to issue exchangeable bonds
    /// filed in the date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn exbd_is_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<ExchangeableBondIssue>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/exbdIsDecsn.json");
        let rows: Vec<ExchangeableBondIssue> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20191212000301");
        assert_eq!(
            rows[0].bd_fta.as_ref().map(Number::as_str),
            Some("6,000,000,000")
        );
    }
}
