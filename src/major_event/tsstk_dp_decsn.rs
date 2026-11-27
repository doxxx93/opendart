//! 자기주식 처분 결정 (`tsstkDpDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "tsstkDpDecsn.json";

/// One 자기주식 처분 결정 report: decisions to dispose of treasury stock, returned by
/// [`Client::tsstk_dp_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct TreasuryStockDisposal {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 처분예정주식(주)(보통주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub dppln_stk_ostk: Option<Number>,
    /// 처분예정주식(주)(기타주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub dppln_stk_estk: Option<Number>,
    /// 처분 대상 주식가격(원)(보통주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub dpstk_prc_ostk: Option<Number>,
    /// 처분 대상 주식가격(원)(기타주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub dpstk_prc_estk: Option<Number>,
    /// 처분예정금액(원)(보통주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub dppln_prc_ostk: Option<Number>,
    /// 처분예정금액(원)(기타주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub dppln_prc_estk: Option<Number>,
    /// 처분예정기간(시작일)
    #[serde(default)]
    pub dpprpd_bgd: String,
    /// 처분예정기간(종료일)
    #[serde(default)]
    pub dpprpd_edd: String,
    /// 처분목적
    #[serde(default)]
    pub dp_pp: String,
    /// 처분방법(시장을 통한 매도(주))
    #[serde(default, deserialize_with = "decode::number")]
    pub dp_m_mkt: Option<Number>,
    /// 처분방법(시간외대량매매(주))
    #[serde(default, deserialize_with = "decode::number")]
    pub dp_m_ovtm: Option<Number>,
    /// 처분방법(장외처분(주))
    #[serde(default, deserialize_with = "decode::number")]
    pub dp_m_otc: Option<Number>,
    /// 처분방법(기타(주))
    #[serde(default, deserialize_with = "decode::number")]
    pub dp_m_etc: Option<Number>,
    /// 위탁투자중개업자
    #[serde(default)]
    pub cs_iv_bk: String,
    /// 처분 전 자기주식 보유현황(배당가능이익 범위 내 취득(주)(보통주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub aq_wtn_div_ostk: Option<Number>,
    /// 처분 전 자기주식 보유현황(배당가능이익 범위 내 취득(주)(비율(%)))
    #[serde(default, deserialize_with = "decode::number")]
    pub aq_wtn_div_ostk_rt: Option<Number>,
    /// 처분 전 자기주식 보유현황(배당가능이익 범위 내 취득(주)(기타주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub aq_wtn_div_estk: Option<Number>,
    /// 처분 전 자기주식 보유현황(배당가능이익 범위 내 취득(주)(비율(%)))
    #[serde(default, deserialize_with = "decode::number")]
    pub aq_wtn_div_estk_rt: Option<Number>,
    /// 처분 전 자기주식 보유현황(기타취득(주)(보통주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub eaq_ostk: Option<Number>,
    /// 처분 전 자기주식 보유현황(기타취득(주)(비율(%)))
    #[serde(default, deserialize_with = "decode::number")]
    pub eaq_ostk_rt: Option<Number>,
    /// 처분 전 자기주식 보유현황(기타취득(주)(기타주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub eaq_estk: Option<Number>,
    /// 처분 전 자기주식 보유현황(기타취득(주)(비율(%)))
    #[serde(default, deserialize_with = "decode::number")]
    pub eaq_estk_rt: Option<Number>,
    /// 처분결정일
    #[serde(default)]
    pub dp_dd: String,
    /// 독립(사외)이사참석여부(참석(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_t: Option<Number>,
    /// 독립(사외)이사참석여부(불참(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_b: Option<Number>,
    /// 감사(독립(사외)이사가 아닌 감사위원)참석여부
    #[serde(default)]
    pub adt_a_atn: String,
    /// 1일 매도 주문수량 한도(보통주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub d1_slodlm_ostk: Option<Number>,
    /// 1일 매도 주문수량 한도(기타주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub d1_slodlm_estk: Option<Number>,
}

impl Client {
    /// 자기주식 처분 결정 (`tsstkDpDecsn.json`): decisions to dispose of treasury stock
    /// filed in the date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn tsstk_dp_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<TreasuryStockDisposal>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/tsstkDpDecsn.json");
        let rows: Vec<TreasuryStockDisposal> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20190429000701");
        assert_eq!(
            rows[0].dppln_stk_ostk.as_ref().map(Number::as_str),
            Some("846")
        );
    }
}
