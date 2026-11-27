//! 주식교환·이전 결정 (`stkExtrDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "stkExtrDecsn.json";

/// One 주식교환·이전 결정 report: decisions on a comprehensive share exchange or
/// transfer, returned by [`Client::stk_extr_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct ShareExchange {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 구분
    #[serde(default)]
    pub extr_sen: String,
    /// 교환ㆍ이전 형태
    #[serde(default)]
    pub extr_stn: String,
    /// 교환ㆍ이전 대상법인(회사명)
    #[serde(default)]
    pub extr_tgcmp_cmpnm: String,
    /// 교환ㆍ이전 대상법인(대표자)
    #[serde(default)]
    pub extr_tgcmp_rp: String,
    /// 교환ㆍ이전 대상법인(주요사업)
    #[serde(default)]
    pub extr_tgcmp_mbsn: String,
    /// 교환ㆍ이전 대상법인(회사와의 관계)
    #[serde(default)]
    pub extr_tgcmp_rl_cmpn: String,
    /// 교환ㆍ이전 대상법인(발행주식총수(주)(보통주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub extr_tgcmp_tisstk_ostk: Option<Number>,
    /// 교환ㆍ이전 대상법인(발행주식총수(주)(종류주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub extr_tgcmp_tisstk_cstk: Option<Number>,
    /// 교환ㆍ이전 대상법인(최근 사업연도 요약재무내용(원)(자산총계))
    #[serde(default, deserialize_with = "decode::number")]
    pub rbsnfdtl_tast: Option<Number>,
    /// 교환ㆍ이전 대상법인(최근 사업연도 요약재무내용(원)(부채총계))
    #[serde(default, deserialize_with = "decode::number")]
    pub rbsnfdtl_tdbt: Option<Number>,
    /// 교환ㆍ이전 대상법인(최근 사업연도 요약재무내용(원)(자본총계))
    #[serde(default, deserialize_with = "decode::number")]
    pub rbsnfdtl_teqt: Option<Number>,
    /// 교환ㆍ이전 대상법인(최근 사업연도 요약재무내용(원)(자본금))
    #[serde(default, deserialize_with = "decode::number")]
    pub rbsnfdtl_cpt: Option<Number>,
    /// 교환ㆍ이전 비율
    #[serde(default)]
    pub extr_rt: String,
    /// 교환ㆍ이전 비율 산출근거
    #[serde(default)]
    pub extr_rt_bs: String,
    /// 외부평가에 관한 사항(외부평가 여부)
    #[serde(default)]
    pub exevl_atn: String,
    /// 외부평가에 관한 사항(근거 및 사유)
    #[serde(default)]
    pub exevl_bs_rs: String,
    /// 외부평가에 관한 사항(외부평가기관의 명칭)
    #[serde(default)]
    pub exevl_intn: String,
    /// 외부평가에 관한 사항(외부평가 기간)
    #[serde(default)]
    pub exevl_pd: String,
    /// 외부평가에 관한 사항(외부평가 의견)
    #[serde(default)]
    pub exevl_op: String,
    /// 교환ㆍ이전 목적
    #[serde(default)]
    pub extr_pp: String,
    /// 교환ㆍ이전일정(교환ㆍ이전계약일)
    #[serde(default)]
    pub extrsc_extrctrd: String,
    /// 교환ㆍ이전일정(주주확정기준일)
    #[serde(default)]
    pub extrsc_shddstd: String,
    /// 교환ㆍ이전일정(주주명부 폐쇄기간(시작일))
    #[serde(default)]
    pub extrsc_shclspd_bgd: String,
    /// 교환ㆍ이전일정(주주명부 폐쇄기간(종료일))
    #[serde(default)]
    pub extrsc_shclspd_edd: String,
    /// 교환ㆍ이전일정(주식교환ㆍ이전 반대의사 통지접수기간(시작일))
    #[serde(default)]
    pub extrsc_extrop_rcpd_bgd: String,
    /// 교환ㆍ이전일정(주식교환ㆍ이전 반대의사 통지접수기간(종료일))
    #[serde(default)]
    pub extrsc_extrop_rcpd_edd: String,
    /// 교환ㆍ이전일정(주주총회 예정일자)
    #[serde(default)]
    pub extrsc_gmtsck_prd: String,
    /// 교환ㆍ이전일정(주식매수청구권 행사기간(시작일))
    #[serde(default)]
    pub extrsc_aprskh_expd_bgd: String,
    /// 교환ㆍ이전일정(주식매수청구권 행사기간(종료일))
    #[serde(default)]
    pub extrsc_aprskh_expd_edd: String,
    /// 교환ㆍ이전일정(구주권제출기간(시작일))
    #[serde(default)]
    pub extrsc_osprpd_bgd: String,
    /// 교환ㆍ이전일정(구주권제출기간(종료일))
    #[serde(default)]
    pub extrsc_osprpd_edd: String,
    /// 교환ㆍ이전일정(매매거래정지예정기간): ① 2019년 12월 08일까지 사용됨
    #[serde(default)]
    pub extrsc_trspprpd: String,
    /// 교환ㆍ이전일정(매매거래정지예정기간(시작일)): ② 2019년 12월 09일부터 추가됨
    #[serde(default)]
    pub extrsc_trspprpd_bgd: String,
    /// 교환ㆍ이전일정(매매거래정지예정기간(종료일)): ② 2019년 12월 09일부터 추가됨
    #[serde(default)]
    pub extrsc_trspprpd_edd: String,
    /// 교환ㆍ이전일정(교환ㆍ이전일자)
    #[serde(default)]
    pub extrsc_extrdt: String,
    /// 교환ㆍ이전일정(신주권교부예정일)
    #[serde(default)]
    pub extrsc_nstkdlprd: String,
    /// 교환ㆍ이전일정(신주의 상장예정일)
    #[serde(default)]
    pub extrsc_nstklstprd: String,
    /// 교환ㆍ이전 후 완전모회사명
    #[serde(default)]
    pub atextr_cpcmpnm: String,
    /// 주식매수청구권에 관한 사항(매수예정가격)
    #[serde(default, deserialize_with = "decode::number")]
    pub aprskh_plnprc: Option<Number>,
    /// 주식매수청구권에 관한 사항(지급예정시기, 지급방법)
    #[serde(default)]
    pub aprskh_pym_plpd_mth: String,
    /// 주식매수청구권에 관한 사항(주식매수청구권 제한 관련 내용)
    #[serde(default)]
    pub aprskh_lmt: String,
    /// 주식매수청구권에 관한 사항(계약에 미치는 효력)
    #[serde(default)]
    pub aprskh_ctref: String,
    /// 우회상장 해당 여부
    #[serde(default)]
    pub bdlst_atn: String,
    /// 타법인의 우회상장 요건 충족 여부
    #[serde(default)]
    pub otcpr_bdlst_sf_atn: String,
    /// 이사회결의일(결정일)
    #[serde(default)]
    pub bddd: String,
    /// 독립(사외)이사참석여부(참석(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_t: Option<Number>,
    /// 독립(사외)이사참석여부(불참(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_b: Option<Number>,
    /// 감사(독립(사외)이사가 아닌 감사위원) 참석여부
    #[serde(default)]
    pub adt_a_atn: String,
    /// 풋옵션 등 계약 체결여부
    #[serde(default)]
    pub popt_ctr_atn: String,
    /// 계약내용
    #[serde(default)]
    pub popt_ctr_cn: String,
    /// 증권신고서 제출대상 여부
    #[serde(default)]
    pub rs_sm_atn: String,
    /// 제출을 면제받은 경우 그 사유
    #[serde(default)]
    pub ex_sm_r: String,
}

impl Client {
    /// 주식교환·이전 결정 (`stkExtrDecsn.json`): decisions on a comprehensive share
    /// exchange or transfer filed in the date range.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn stk_extr_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<ShareExchange>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/stkExtrDecsn.json");
        let rows: Vec<ShareExchange> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20190417000386");
        assert_eq!(
            rows[0].extr_tgcmp_tisstk_ostk.as_ref().map(Number::as_str),
            Some("886,913")
        );
    }
}
