//! 회사분할 결정 (`cmpDvDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "cmpDvDecsn.json";

/// One 회사분할 결정 report: decisions to split the company, returned by
/// [`Client::cmp_dv_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct CorporateSplit {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 분할방법
    #[serde(default)]
    pub dv_mth: String,
    /// 분할의 중요영향 및 효과
    #[serde(default)]
    pub dv_impef: String,
    /// 분할비율
    #[serde(default)]
    pub dv_rt: String,
    /// 분할로 이전할 사업 및 재산의 내용
    #[serde(default)]
    pub dv_trfbsnprt_cn: String,
    /// 분할 후 존속회사(회사명)
    #[serde(default)]
    pub atdv_excmp_cmpnm: String,
    /// 분할 후 존속회사(분할후 재무내용(원)(자산총계))
    #[serde(default, deserialize_with = "decode::number")]
    pub atdvfdtl_tast: Option<Number>,
    /// 분할 후 존속회사(분할후 재무내용(원)(부채총계))
    #[serde(default, deserialize_with = "decode::number")]
    pub atdvfdtl_tdbt: Option<Number>,
    /// 분할 후 존속회사(분할후 재무내용(원)(자본총계))
    #[serde(default, deserialize_with = "decode::number")]
    pub atdvfdtl_teqt: Option<Number>,
    /// 분할 후 존속회사(분할후 재무내용(원)(자본금))
    #[serde(default, deserialize_with = "decode::number")]
    pub atdvfdtl_cpt: Option<Number>,
    /// 분할 후 존속회사(분할후 재무내용(원)(현재기준))
    #[serde(default)]
    pub atdvfdtl_std: String,
    /// 분할 후 존속회사(존속사업부문 최근 사업연도매출액(원))
    #[serde(default, deserialize_with = "decode::number")]
    pub atdv_excmp_exbsn_rsl: Option<Number>,
    /// 분할 후 존속회사(주요사업)
    #[serde(default)]
    pub atdv_excmp_mbsn: String,
    /// 분할 후 존속회사(분할 후 상장유지 여부)
    #[serde(default)]
    pub atdv_excmp_atdv_lstmn_atn: String,
    /// 분할설립회사(회사명)
    #[serde(default)]
    pub dvfcmp_cmpnm: String,
    /// 분할설립회사(설립시 재무내용(원)(자산총계))
    #[serde(default, deserialize_with = "decode::number")]
    pub ffdtl_tast: Option<Number>,
    /// 분할설립회사(설립시 재무내용(원)(부채총계))
    #[serde(default, deserialize_with = "decode::number")]
    pub ffdtl_tdbt: Option<Number>,
    /// 분할설립회사(설립시 재무내용(원)(자본총계))
    #[serde(default, deserialize_with = "decode::number")]
    pub ffdtl_teqt: Option<Number>,
    /// 분할설립회사(설립시 재무내용(원)(자본금))
    #[serde(default, deserialize_with = "decode::number")]
    pub ffdtl_cpt: Option<Number>,
    /// 분할설립회사(설립시 재무내용(원)(현재기준))
    #[serde(default)]
    pub ffdtl_std: String,
    /// 분할설립회사(신설사업부문 최근 사업연도 매출액(원))
    #[serde(default, deserialize_with = "decode::number")]
    pub dvfcmp_nbsn_rsl: Option<Number>,
    /// 분할설립회사(주요사업)
    #[serde(default)]
    pub dvfcmp_mbsn: String,
    /// 분할설립회사(재상장신청 여부)
    #[serde(default)]
    pub dvfcmp_rlst_atn: String,
    /// 감자에 관한 사항(감자비율(%))
    #[serde(default, deserialize_with = "decode::number")]
    pub abcr_crrt: Option<Number>,
    /// 감자에 관한 사항(구주권 제출기간(시작일))
    #[serde(default)]
    pub abcr_osprpd_bgd: String,
    /// 감자에 관한 사항(구주권 제출기간(종료일))
    #[serde(default)]
    pub abcr_osprpd_edd: String,
    /// 감자에 관한 사항(매매거래정지 예정기간(시작일))
    #[serde(default)]
    pub abcr_trspprpd_bgd: String,
    /// 감자에 관한 사항(매매거래정지 예정기간(종료일))
    #[serde(default)]
    pub abcr_trspprpd_edd: String,
    /// 감자에 관한 사항(신주배정조건)
    #[serde(default)]
    pub abcr_nstkascnd: String,
    /// 감자에 관한 사항(주주 주식수 비례여부 및 사유)
    #[serde(default)]
    pub abcr_shstkcnt_rt_at_rs: String,
    /// 감자에 관한 사항(신주배정기준일)
    #[serde(default)]
    pub abcr_nstkasstd: String,
    /// 감자에 관한 사항(신주권교부예정일)
    #[serde(default)]
    pub abcr_nstkdlprd: String,
    /// 감자에 관한 사항(신주의 상장예정일)
    #[serde(default)]
    pub abcr_nstklstprd: String,
    /// 주주총회 예정일
    #[serde(default)]
    pub gmtsck_prd: String,
    /// 채권자 이의제출기간(시작일)
    #[serde(default)]
    pub cdobprpd_bgd: String,
    /// 채권자 이의제출기간(종료일)
    #[serde(default)]
    pub cdobprpd_edd: String,
    /// 분할기일
    #[serde(default)]
    pub dvdt: String,
    /// 분할등기 예정일
    #[serde(default)]
    pub dvrgsprd: String,
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
    /// 회사분할 결정 (`cmpDvDecsn.json`): decisions to split the company filed in the
    /// date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn cmp_dv_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<CorporateSplit>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/cmpDvDecsn.json");
        let rows: Vec<CorporateSplit> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20190827000439");
        assert_eq!(
            rows[0].atdvfdtl_tast.as_ref().map(Number::as_str),
            Some("5,748,009,034,635")
        );
    }
}
