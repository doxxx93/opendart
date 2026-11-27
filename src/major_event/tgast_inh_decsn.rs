//! 유형자산 양수 결정 (`tgastInhDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "tgastInhDecsn.json";

/// One 유형자산 양수 결정 report: decisions to acquire tangible assets, returned by
/// [`Client::tgast_inh_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct TangibleAssetAcquisition {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 자산구분
    #[serde(default)]
    pub ast_sen: String,
    /// 자산명
    #[serde(default)]
    pub ast_nm: String,
    /// 양수내역(양수금액(원))
    #[serde(default, deserialize_with = "decode::number")]
    pub inhdtl_inhprc: Option<Number>,
    /// 양수내역(자산총액(원))
    #[serde(default, deserialize_with = "decode::number")]
    pub inhdtl_tast: Option<Number>,
    /// 양수내역(자산총액대비(%))
    #[serde(default, deserialize_with = "decode::number")]
    pub inhdtl_tast_vs: Option<Number>,
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
    /// 양수예정일자(등기예정일)
    #[serde(default)]
    pub inh_prd_rgs_prd: String,
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
    /// 거래대금지급
    #[serde(default)]
    pub dl_pym: String,
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
    /// 유형자산 양수 결정 (`tgastInhDecsn.json`): decisions to acquire tangible assets
    /// filed in the date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn tgast_inh_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<TangibleAssetAcquisition>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/tgastInhDecsn.json");
        let rows: Vec<TangibleAssetAcquisition> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20191011000131");
        assert_eq!(
            rows[0].inhdtl_inhprc.as_ref().map(Number::as_str),
            Some("6,100,000,000")
        );
    }
}
