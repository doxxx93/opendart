//! 주권 관련 사채권 양도 결정 (`stkrtbdTrfDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "stkrtbdTrfDecsn.json";

/// One 주권 관련 사채권 양도 결정 report: decisions to transfer equity-linked bonds,
/// returned by [`Client::stkrtbd_trf_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct EquityLinkedBondTransfer {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 주권 관련 사채권의 종류
    #[serde(default)]
    pub stkrtbd_kndn: String,
    /// 주권 관련 사채권의 종류(회차)
    #[serde(default)]
    pub tm: String,
    /// 주권 관련 사채권의 종류(종류)
    #[serde(default)]
    pub knd: String,
    /// 취득일자
    #[serde(default)]
    pub aqd: String,
    /// 사채권 발행회사(회사명)
    #[serde(default)]
    pub bdiscmp_cmpnm: String,
    /// 사채권 발행회사(국적)
    #[serde(default)]
    pub bdiscmp_nt: String,
    /// 사채권 발행회사(대표자)
    #[serde(default)]
    pub bdiscmp_rp: String,
    /// 사채권 발행회사(자본금(원))
    #[serde(default, deserialize_with = "decode::number")]
    pub bdiscmp_cpt: Option<Number>,
    /// 사채권 발행회사(회사와 관계)
    #[serde(default)]
    pub bdiscmp_rl_cmpn: String,
    /// 사채권 발행회사(발행주식 총수(주))
    #[serde(default, deserialize_with = "decode::number")]
    pub bdiscmp_tisstk: Option<Number>,
    /// 사채권 발행회사(주요사업)
    #[serde(default)]
    pub bdiscmp_mbsn: String,
    /// 양도내역(사채의 권면(전자등록)총액(원))
    #[serde(default, deserialize_with = "decode::number")]
    pub trfdtl_bd_fta: Option<Number>,
    /// 양도내역(양도금액(원)(A))
    #[serde(default, deserialize_with = "decode::number")]
    pub trfdtl_trfprc: Option<Number>,
    /// 양도내역(총자산(원)(B))
    #[serde(default, deserialize_with = "decode::number")]
    pub trfdtl_tast: Option<Number>,
    /// 양도내역(총자산대비(%)(A/B))
    #[serde(default, deserialize_with = "decode::number")]
    pub trfdtl_tast_vs: Option<Number>,
    /// 양도내역(자기자본(원)(C))
    #[serde(default, deserialize_with = "decode::number")]
    pub trfdtl_ecpt: Option<Number>,
    /// 양도내역(자기자본대비(%)(A/C))
    #[serde(default, deserialize_with = "decode::number")]
    pub trfdtl_ecpt_vs: Option<Number>,
    /// 양도목적
    #[serde(default)]
    pub trf_pp: String,
    /// 양도예정일자
    #[serde(default)]
    pub trf_prd: String,
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
    /// 이사회결의일(결정일)
    #[serde(default)]
    pub bddd: String,
    /// 독립(사외)이사 참석여부(참석(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_t: Option<Number>,
    /// 독립(사외)이사 참석여부(불참(명))
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
    /// 주권 관련 사채권 양도 결정 (`stkrtbdTrfDecsn.json`): decisions to transfer
    /// equity-linked bonds filed in the date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn stkrtbd_trf_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<EquityLinkedBondTransfer>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/stkrtbdTrfDecsn.json");
        let rows: Vec<EquityLinkedBondTransfer> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20190730000247");
        assert_eq!(
            rows[0].bdiscmp_cpt.as_ref().map(Number::as_str),
            Some("3,022,500,000")
        );
    }
}
