//! 자기주식 취득 결정 (`tsstkAqDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "tsstkAqDecsn.json";

/// One 자기주식 취득 결정 report: decisions to acquire treasury stock, returned by
/// [`Client::tsstk_aq_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct TreasuryStockAcquisition {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 취득예정주식(주)(보통주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub aqpln_stk_ostk: Option<Number>,
    /// 취득예정주식(주)(기타주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub aqpln_stk_estk: Option<Number>,
    /// 취득예정금액(원)(보통주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub aqpln_prc_ostk: Option<Number>,
    /// 취득예정금액(원)(기타주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub aqpln_prc_estk: Option<Number>,
    /// 취득예상기간(시작일)
    #[serde(default)]
    pub aqexpd_bgd: String,
    /// 취득예상기간(종료일)
    #[serde(default)]
    pub aqexpd_edd: String,
    /// 보유예상기간(시작일)
    #[serde(default)]
    pub hdexpd_bgd: String,
    /// 보유예상기간(종료일)
    #[serde(default)]
    pub hdexpd_edd: String,
    /// 취득목적
    #[serde(default)]
    pub aq_pp: String,
    /// 취득방법
    #[serde(default)]
    pub aq_mth: String,
    /// 위탁투자중개업자
    #[serde(default)]
    pub cs_iv_bk: String,
    /// 취득 전 자기주식 보유현황(배당가능이익 범위 내 취득(주)(보통주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub aq_wtn_div_ostk: Option<Number>,
    /// 취득 전 자기주식 보유현황(배당가능이익 범위 내 취득(주)(비율(%)))
    #[serde(default, deserialize_with = "decode::number")]
    pub aq_wtn_div_ostk_rt: Option<Number>,
    /// 취득 전 자기주식 보유현황(배당가능이익 범위 내 취득(주)(기타주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub aq_wtn_div_estk: Option<Number>,
    /// 취득 전 자기주식 보유현황(배당가능이익 범위 내 취득(주)(비율(%)))
    #[serde(default, deserialize_with = "decode::number")]
    pub aq_wtn_div_estk_rt: Option<Number>,
    /// 취득 전 자기주식 보유현황(기타취득(주)(보통주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub eaq_ostk: Option<Number>,
    /// 취득 전 자기주식 보유현황(기타취득(주)(비율(%)))
    #[serde(default, deserialize_with = "decode::number")]
    pub eaq_ostk_rt: Option<Number>,
    /// 취득 전 자기주식 보유현황(기타취득(주)(기타주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub eaq_estk: Option<Number>,
    /// 취득 전 자기주식 보유현황(기타취득(주)(비율(%)))
    #[serde(default, deserialize_with = "decode::number")]
    pub eaq_estk_rt: Option<Number>,
    /// 취득결정일
    #[serde(default)]
    pub aq_dd: String,
    /// 독립(사외)이사참석여부(참석(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_t: Option<Number>,
    /// 독립(사외)이사참석여부(불참(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_b: Option<Number>,
    /// 감사(독립(사외)이사가 아닌 감사위원)참석여부
    #[serde(default)]
    pub adt_a_atn: String,
    /// 1일 매수 주문수량 한도(보통주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub d1_prodlm_ostk: Option<Number>,
    /// 1일 매수 주문수량 한도(기타주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub d1_prodlm_estk: Option<Number>,
}

impl Client {
    /// 자기주식 취득 결정 (`tsstkAqDecsn.json`): decisions to acquire treasury stock
    /// filed in the date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn tsstk_aq_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<TreasuryStockAcquisition>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/tsstkAqDecsn.json");
        let rows: Vec<TreasuryStockAcquisition> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].rcept_no, "20191204000063");
        assert_eq!(
            rows[0].aqpln_stk_ostk.as_ref().map(Number::as_str),
            Some("2,136,681")
        );
    }
}
