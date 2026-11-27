//! 회사분할합병 결정 (`cmpDvmgDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "cmpDvmgDecsn.json";

/// One 회사분할합병 결정 report: decisions on a split-merger, returned by
/// [`Client::cmp_dvmg_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct SplitMerger {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 분할합병 방법
    #[serde(default)]
    pub dvmg_mth: String,
    /// 분할합병의 중요영향 및 효과
    #[serde(default)]
    pub dvmg_impef: String,
    /// 분할에 관한 사항(분할로 이전할 사업 및 재산의 내용)
    #[serde(default)]
    pub dv_trfbsnprt_cn: String,
    /// 분할에 관한 사항(분할 후 존속회사(회사명))
    #[serde(default)]
    pub atdv_excmp_cmpnm: String,
    /// 분할에 관한 사항(분할 후 존속회사(분할후 재무내용(원)(자산총계)))
    #[serde(default, deserialize_with = "decode::number")]
    pub atdvfdtl_tast: Option<Number>,
    /// 분할에 관한 사항(분할 후 존속회사(분할후 재무내용(원)(부채총계)))
    #[serde(default, deserialize_with = "decode::number")]
    pub atdvfdtl_tdbt: Option<Number>,
    /// 분할에 관한 사항(분할 후 존속회사(분할후 재무내용(원)(자본총계)))
    #[serde(default, deserialize_with = "decode::number")]
    pub atdvfdtl_teqt: Option<Number>,
    /// 분할에 관한 사항(분할 후 존속회사(분할후 재무내용(원)(자본금)))
    #[serde(default, deserialize_with = "decode::number")]
    pub atdvfdtl_cpt: Option<Number>,
    /// 분할에 관한 사항(분할 후 존속회사(분할후 재무내용(원)(현재기준)))
    #[serde(default)]
    pub atdvfdtl_std: String,
    /// 분할에 관한 사항(분할 후 존속회사(존속사업부문 최근 사업연도매출액(원)))
    #[serde(default, deserialize_with = "decode::number")]
    pub atdv_excmp_exbsn_rsl: Option<Number>,
    /// 분할에 관한 사항(분할 후 존속회사(주요사업))
    #[serde(default)]
    pub atdv_excmp_mbsn: String,
    /// 분할에 관한 사항(분할 후 존속회사(분할 후 상장유지 여부))
    #[serde(default)]
    pub atdv_excmp_atdv_lstmn_atn: String,
    /// 분할에 관한 사항(분할설립 회사(회사명))
    #[serde(default)]
    pub dvfcmp_cmpnm: String,
    /// 분할에 관한 사항(분할설립 회사(설립시 재무내용(원)(자산총계)))
    #[serde(default, deserialize_with = "decode::number")]
    pub ffdtl_tast: Option<Number>,
    /// 분할에 관한 사항(분할설립 회사(설립시 재무내용(원)(부채총계)))
    #[serde(default, deserialize_with = "decode::number")]
    pub ffdtl_tdbt: Option<Number>,
    /// 분할에 관한 사항(분할설립 회사(설립시 재무내용(원)(자본총계)))
    #[serde(default, deserialize_with = "decode::number")]
    pub ffdtl_teqt: Option<Number>,
    /// 분할에 관한 사항(분할설립 회사(설립시 재무내용(원)(자본금)))
    #[serde(default, deserialize_with = "decode::number")]
    pub ffdtl_cpt: Option<Number>,
    /// 분할에 관한 사항(분할설립 회사(설립시 재무내용(원)(현재기준)))
    #[serde(default)]
    pub ffdtl_std: String,
    /// 분할에 관한 사항(분할설립 회사(신설사업부문 최근 사업연도 매출액(원)))
    #[serde(default, deserialize_with = "decode::number")]
    pub dvfcmp_nbsn_rsl: Option<Number>,
    /// 분할에 관한 사항(분할설립 회사(주요사업))
    #[serde(default)]
    pub dvfcmp_mbsn: String,
    /// 분할에 관한 사항(분할설립 회사(분할후 상장유지여부))
    #[serde(default)]
    pub dvfcmp_atdv_lstmn_at: String,
    /// 분할에 관한 사항(감자에 관한 사항(감자비율(%)))
    #[serde(default, deserialize_with = "decode::number")]
    pub abcr_crrt: Option<Number>,
    /// 분할에 관한 사항(감자에 관한 사항(구주권 제출기간(시작일)))
    #[serde(default)]
    pub abcr_osprpd_bgd: String,
    /// 분할에 관한 사항(감자에 관한 사항(구주권 제출기간(종료일)))
    #[serde(default)]
    pub abcr_osprpd_edd: String,
    /// 분할에 관한 사항(감자에 관한 사항(매매거래정지 예정기간(시작일)))
    #[serde(default)]
    pub abcr_trspprpd_bgd: String,
    /// 분할에 관한 사항(감자에 관한 사항(매매거래정지 예정기간(종료일)))
    #[serde(default)]
    pub abcr_trspprpd_edd: String,
    /// 분할에 관한 사항(감자에 관한 사항(신주배정조건))
    #[serde(default)]
    pub abcr_nstkascnd: String,
    /// 분할에 관한 사항(감자에 관한 사항(주주 주식수 비례여부 및 사유))
    #[serde(default)]
    pub abcr_shstkcnt_rt_at_rs: String,
    /// 분할에 관한 사항(감자에 관한 사항(신주배정기준일))
    #[serde(default)]
    pub abcr_nstkasstd: String,
    /// 분할에 관한 사항(감자에 관한 사항(신주권교부예정일))
    #[serde(default)]
    pub abcr_nstkdlprd: String,
    /// 분할에 관한 사항(감자에 관한 사항(신주의 상장예정일))
    #[serde(default)]
    pub abcr_nstklstprd: String,
    /// 합병에 관한 사항(합병형태)
    #[serde(default)]
    pub mg_stn: String,
    /// 합병에 관한 사항(합병상대 회사(회사명))
    #[serde(default)]
    pub mgptncmp_cmpnm: String,
    /// 합병에 관한 사항(합병상대 회사(주요사업))
    #[serde(default)]
    pub mgptncmp_mbsn: String,
    /// 합병에 관한 사항(합병상대 회사(회사와의 관계))
    #[serde(default)]
    pub mgptncmp_rl_cmpn: String,
    /// 합병에 관한 사항(합병상대 회사(최근 사업연도 재무내용(원)(자산총계)))
    #[serde(default, deserialize_with = "decode::number")]
    pub rbsnfdtl_tast: Option<Number>,
    /// 합병에 관한 사항(합병상대 회사(최근 사업연도 재무내용(원)(부채총계)))
    #[serde(default, deserialize_with = "decode::number")]
    pub rbsnfdtl_tdbt: Option<Number>,
    /// 합병에 관한 사항(합병상대 회사(최근 사업연도 재무내용(원)(자본총계)))
    #[serde(default, deserialize_with = "decode::number")]
    pub rbsnfdtl_teqt: Option<Number>,
    /// 합병에 관한 사항(합병상대 회사(최근 사업연도 재무내용(원)(자본금)))
    #[serde(default, deserialize_with = "decode::number")]
    pub rbsnfdtl_cpt: Option<Number>,
    /// 합병에 관한 사항(합병상대 회사(최근 사업연도 재무내용(원)(매출액)))
    #[serde(default, deserialize_with = "decode::number")]
    pub rbsnfdtl_sl: Option<Number>,
    /// 합병에 관한 사항(합병상대 회사(최근 사업연도 재무내용(원)(당기순이익)))
    #[serde(default, deserialize_with = "decode::number")]
    pub rbsnfdtl_nic: Option<Number>,
    /// 합병에 관한 사항(합병상대 회사(외부감사 여부(기관명)))
    #[serde(default)]
    pub eadtat_intn: String,
    /// 합병에 관한 사항(합병상대 회사(외부감사 여부(감사의견)))
    #[serde(default)]
    pub eadtat_op: String,
    /// 합병에 관한 사항(분할합병신주의 종류와 수(주)(보통주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub dvmgnstk_ostk_cnt: Option<Number>,
    /// 합병에 관한 사항(분할합병신주의 종류와 수(주)(종류주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub dvmgnstk_cstk_cnt: Option<Number>,
    /// 합병에 관한 사항(합병신설 회사(회사명))
    #[serde(default)]
    pub nmgcmp_cmpnm: String,
    /// 합병에 관한 사항(합병신설 회사(자본금(원)))
    #[serde(default, deserialize_with = "decode::number")]
    pub nmgcmp_cpt: Option<Number>,
    /// 합병에 관한 사항(합병신설 회사(주요사업))
    #[serde(default)]
    pub nmgcmp_mbsn: String,
    /// 합병에 관한 사항(합병신설 회사(재상장신청 여부))
    #[serde(default)]
    pub nmgcmp_rlst_atn: String,
    /// 분할합병비율
    #[serde(default)]
    pub dvmg_rt: String,
    /// 분할합병비율 산출근거
    #[serde(default)]
    pub dvmg_rt_bs: String,
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
    /// 분할합병일정(분할합병계약일)
    #[serde(default)]
    pub dvmgsc_dvmgctrd: String,
    /// 분할합병일정(주주확정기준일)
    #[serde(default)]
    pub dvmgsc_shddstd: String,
    /// 분할합병일정(주주명부 폐쇄기간(시작일))
    #[serde(default)]
    pub dvmgsc_shclspd_bgd: String,
    /// 분할합병일정(주주명부 폐쇄기간(종료일))
    #[serde(default)]
    pub dvmgsc_shclspd_edd: String,
    /// 분할합병일정(분할합병반대의사통지 접수기간(시작일))
    #[serde(default)]
    pub dvmgsc_dvmgop_rcpd_bgd: String,
    /// 분할합병일정(분할합병반대의사통지 접수기간(종료일))
    #[serde(default)]
    pub dvmgsc_dvmgop_rcpd_edd: String,
    /// 분할합병일정(주주총회예정일자)
    #[serde(default)]
    pub dvmgsc_gmtsck_prd: String,
    /// 분할합병일정(주식매수청구권 행사기간(시작일))
    #[serde(default)]
    pub dvmgsc_aprskh_expd_bgd: String,
    /// 분할합병일정(주식매수청구권 행사기간(종료일))
    #[serde(default)]
    pub dvmgsc_aprskh_expd_edd: String,
    /// 분할합병일정(채권자 이의 제출기간(시작일))
    #[serde(default)]
    pub dvmgsc_cdobprpd_bgd: String,
    /// 분할합병일정(채권자 이의 제출기간(종료일))
    #[serde(default)]
    pub dvmgsc_cdobprpd_edd: String,
    /// 분할합병일정(분할합병기일)
    #[serde(default)]
    pub dvmgsc_dvmgdt: String,
    /// 분할합병일정(종료보고 총회일)
    #[serde(default)]
    pub dvmgsc_ergmd: String,
    /// 분할합병일정(분할합병등기예정일)
    #[serde(default)]
    pub dvmgsc_dvmgrgsprd: String,
    /// 우회상장 해당 여부
    #[serde(default)]
    pub bdlst_atn: String,
    /// 타법인의 우회상장 요건 충족여부
    #[serde(default)]
    pub otcpr_bdlst_sf_atn: String,
    /// 주식매수청구권에 관한 사항(행사요건)
    #[serde(default)]
    pub aprskh_exrq: String,
    /// 주식매수청구권에 관한 사항(매수예정가격)
    #[serde(default, deserialize_with = "decode::number")]
    pub aprskh_plnprc: Option<Number>,
    /// 주식매수청구권에 관한 사항(행사절차, 방법, 기간, 장소)
    #[serde(default)]
    pub aprskh_ex_pc_mth_pd_pl: String,
    /// 주식매수청구권에 관한 사항(지급예정시기, 지급방법)
    #[serde(default)]
    pub aprskh_pym_plpd_mth: String,
    /// 주식매수청구권에 관한 사항(주식매수청구권 제한 관련 내용)
    #[serde(default)]
    pub aprskh_lmt: String,
    /// 주식매수청구권에 관한 사항(계약에 미치는 효력)
    #[serde(default)]
    pub aprskh_ctref: String,
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
    /// 회사분할합병 결정 (`cmpDvmgDecsn.json`): decisions on a split-merger filed in
    /// the date range.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn cmp_dvmg_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<SplitMerger>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/cmpDvmgDecsn.json");
        let rows: Vec<SplitMerger> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20190416000471");
        assert_eq!(
            rows[0].atdvfdtl_tast.as_ref().map(Number::as_str),
            Some("3,844,281,479,628")
        );
    }
}
