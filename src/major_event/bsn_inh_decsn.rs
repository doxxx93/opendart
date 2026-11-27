//! 영업양수 결정 (`bsnInhDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "bsnInhDecsn.json";

/// One 영업양수 결정 report: decisions to acquire a business, returned by
/// [`Client::bsn_inh_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct BusinessAcquisition {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 양수영업
    #[serde(default)]
    pub inh_bsn: String,
    /// 양수영업 주요내용
    #[serde(default)]
    pub inh_bsn_mc: String,
    /// 양수가액(원)
    #[serde(default, deserialize_with = "decode::number")]
    pub inh_prc: Option<Number>,
    /// 영업전부의 양수 여부
    #[serde(default)]
    pub absn_inh_atn: String,
    /// 재무내용(원)(자산액(양수대상 영업부문(A)))
    #[serde(default, deserialize_with = "decode::number")]
    pub ast_inh_bsn: Option<Number>,
    /// 재무내용(원)(자산액(당사전체(B)))
    #[serde(default, deserialize_with = "decode::number")]
    pub ast_cmp_all: Option<Number>,
    /// 재무내용(원)(자산액(비중(%)(A/B)))
    #[serde(default, deserialize_with = "decode::number")]
    pub ast_rt: Option<Number>,
    /// 재무내용(원)(매출액(양수대상 영업부문(A)))
    #[serde(default, deserialize_with = "decode::number")]
    pub sl_inh_bsn: Option<Number>,
    /// 재무내용(원)(매출액(당사전체(B)))
    #[serde(default, deserialize_with = "decode::number")]
    pub sl_cmp_all: Option<Number>,
    /// 재무내용(원)(매출액(비중(%)(A/B)))
    #[serde(default, deserialize_with = "decode::number")]
    pub sl_rt: Option<Number>,
    /// 재무내용(원)(부채액(양수대상 영업부문(A)))
    #[serde(default, deserialize_with = "decode::number")]
    pub dbt_inh_bsn: Option<Number>,
    /// 재무내용(원)(부채액(당사전체(B)))
    #[serde(default, deserialize_with = "decode::number")]
    pub dbt_cmp_all: Option<Number>,
    /// 재무내용(원)(부채액(비중(%)(A/B)))
    #[serde(default, deserialize_with = "decode::number")]
    pub dbt_rt: Option<Number>,
    /// 양수목적
    #[serde(default)]
    pub inh_pp: String,
    /// 양수영향
    #[serde(default)]
    pub inh_af: String,
    /// 양수예정일자(계약체결일)
    #[serde(default)]
    pub inh_prd_ctr_cnsd: String,
    /// 양수예정일자(양수기준일)
    #[serde(default)]
    pub inh_prd_inh_std: String,
    /// 거래상대방(회사명(성명))
    #[serde(default)]
    pub dlptn_cmpnm: String,
    /// 거래상대방(자본금(원))
    #[serde(default, deserialize_with = "decode::number")]
    pub dlptn_cpt: Option<Number>,
    /// 거래상대방(주요사업)
    #[serde(default)]
    pub dlptn_mbsn: String,
    /// 거래상대방(본점소재지(주소))
    #[serde(default)]
    pub dlptn_hoadd: String,
    /// 거래상대방(회사와의 관계)
    #[serde(default)]
    pub dlptn_rl_cmpn: String,
    /// 양수대금지급
    #[serde(default)]
    pub inh_pym: String,
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
    /// 주주총회 특별결의 여부
    #[serde(default)]
    pub gmtsck_spd_atn: String,
    /// 주주총회 예정일자
    #[serde(default)]
    pub gmtsck_prd: String,
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
    /// 우회상장 해당 여부
    #[serde(default)]
    pub bdlst_atn: String,
    /// 향후 6월이내 제3자배정 증자 등 계획
    #[serde(default)]
    pub n6m_tpai_plann: String,
    /// 타법인의 우회상장 요건 충족여부
    #[serde(default)]
    pub otcpr_bdlst_sf_atn: String,
    /// 공정거래위원회 신고대상 여부
    #[serde(default)]
    pub ftc_stt_atn: String,
    /// 풋옵션 등 계약 체결여부
    #[serde(default)]
    pub popt_ctr_atn: String,
    /// 계약내용
    #[serde(default)]
    pub popt_ctr_cn: String,
}

impl Client {
    /// 영업양수 결정 (`bsnInhDecsn.json`): decisions to acquire a business filed in
    /// the date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn bsn_inh_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<BusinessAcquisition>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/bsnInhDecsn.json");
        let rows: Vec<BusinessAcquisition> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20190430001369");
        assert_eq!(
            rows[0].inh_prc.as_ref().map(Number::as_str),
            Some("5,050,000,000")
        );
    }
}
