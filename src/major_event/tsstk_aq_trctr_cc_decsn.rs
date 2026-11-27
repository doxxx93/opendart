//! 자기주식취득 신탁계약 해지 결정 (`tsstkAqTrctrCcDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "tsstkAqTrctrCcDecsn.json";

/// One 자기주식취득 신탁계약 해지 결정 report: decisions to end a trust contract for
/// acquiring treasury stock, returned by [`Client::tsstk_aq_trctr_cc_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct TreasuryTrustTermination {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 계약금액(원)(해지 전)
    #[serde(default, deserialize_with = "decode::number")]
    pub ctr_prc_bfcc: Option<Number>,
    /// 계약금액(원)(해지 후)
    #[serde(default, deserialize_with = "decode::number")]
    pub ctr_prc_atcc: Option<Number>,
    /// 해지 전 계약기간(시작일)
    #[serde(default)]
    pub ctr_pd_bfcc_bgd: String,
    /// 해지 전 계약기간(종료일)
    #[serde(default)]
    pub ctr_pd_bfcc_edd: String,
    /// 해지목적
    #[serde(default)]
    pub cc_pp: String,
    /// 해지기관
    #[serde(default)]
    pub cc_int: String,
    /// 해지예정일자
    #[serde(default)]
    pub cc_prd: String,
    /// 해지후 신탁재산의 반환방법
    #[serde(default)]
    pub tp_rm_atcc: String,
    /// 해지 전 자기주식 보유현황(배당가능범위 내 취득(주)(보통주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub aq_wtn_div_ostk: Option<Number>,
    /// 해지 전 자기주식 보유현황(배당가능범위 내 취득(주)(비율(%)))
    #[serde(default, deserialize_with = "decode::number")]
    pub aq_wtn_div_ostk_rt: Option<Number>,
    /// 해지 전 자기주식 보유현황(배당가능범위 내 취득(주)(기타주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub aq_wtn_div_estk: Option<Number>,
    /// 해지 전 자기주식 보유현황(배당가능범위 내 취득(주)(비율(%)))
    #[serde(default, deserialize_with = "decode::number")]
    pub aq_wtn_div_estk_rt: Option<Number>,
    /// 해지 전 자기주식 보유현황(기타취득(주)(보통주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub eaq_ostk: Option<Number>,
    /// 해지 전 자기주식 보유현황(기타취득(주)(비율(%)))
    #[serde(default, deserialize_with = "decode::number")]
    pub eaq_ostk_rt: Option<Number>,
    /// 해지 전 자기주식 보유현황(기타취득(주)(기타주식))
    #[serde(default, deserialize_with = "decode::number")]
    pub eaq_estk: Option<Number>,
    /// 해지 전 자기주식 보유현황(기타취득(주)(비율(%)))
    #[serde(default, deserialize_with = "decode::number")]
    pub eaq_estk_rt: Option<Number>,
    /// 이사회결의일(결정일)
    #[serde(default)]
    pub bddd: String,
    /// 독립(사외)이사참석여부(참석(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_t: Option<Number>,
    /// 독립(사외)이사참석여부(불참(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_b: Option<Number>,
    /// 감사(독립(사외)이사가 아닌 감사위원)참석여부
    #[serde(default)]
    pub adt_a_atn: String,
}

impl Client {
    /// 자기주식취득 신탁계약 해지 결정 (`tsstkAqTrctrCcDecsn.json`): decisions to end a trust
    /// contract for acquiring treasury stock filed in the date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn tsstk_aq_trctr_cc_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<TreasuryTrustTermination>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/tsstkAqTrctrCcDecsn.json");
        let rows: Vec<TreasuryTrustTermination> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].rcept_no, "20190909001077");
        assert_eq!(
            rows[0].ctr_prc_bfcc.as_ref().map(Number::as_str),
            Some("200,000,000,000")
        );
    }
}
