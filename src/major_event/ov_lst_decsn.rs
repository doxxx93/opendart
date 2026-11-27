//! 해외 증권시장 주권등 상장 결정 (`ovLstDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "ovLstDecsn.json";

/// One 해외 증권시장 주권등 상장 결정 report: decisions to list shares on an overseas
/// market, returned by [`Client::ov_lst_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct OverseasListingDecision {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 상장예정주식 종류ㆍ수(주)(보통주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub lstprstk_ostk_cnt: Option<Number>,
    /// 상장예정주식 종류ㆍ수(주)(기타주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub lstprstk_estk_cnt: Option<Number>,
    /// 발행주식 총수(주)(보통주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub tisstk_ostk: Option<Number>,
    /// 발행주식 총수(주)(기타주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub tisstk_estk: Option<Number>,
    /// 공모방법(신주발행 (주))
    #[serde(default, deserialize_with = "decode::number")]
    pub psmth_nstk_sl: Option<Number>,
    /// 공모방법(구주매출 (주))
    #[serde(default, deserialize_with = "decode::number")]
    pub psmth_ostk_sl: Option<Number>,
    /// 자금조달(신주발행) 목적
    #[serde(default)]
    pub fdpp: String,
    /// 상장증권(원주상장 (주))
    #[serde(default, deserialize_with = "decode::number")]
    pub lststk_orlst: Option<Number>,
    /// 상장증권(DR상장 (주))
    #[serde(default, deserialize_with = "decode::number")]
    pub lststk_drlst: Option<Number>,
    /// 상장거래소(소재국가)
    #[serde(default)]
    pub lstex_nt: String,
    /// 해외상장목적
    #[serde(default)]
    pub lstpp: String,
    /// 상장예정일자
    #[serde(default)]
    pub lstprd: String,
    /// 이사회결의일(결정일)
    #[serde(default)]
    pub bddd: String,
    /// 독립(사외)이사 참석여부(참석(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_t: Option<Number>,
    /// 독립(사외)이사 참석여부(불참(명))
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_b: Option<Number>,
    /// 감사(감사위원)참석여부
    #[serde(default)]
    pub adt_a_atn: String,
}

impl Client {
    /// 해외 증권시장 주권등 상장 결정 (`ovLstDecsn.json`): decisions to list shares on an
    /// overseas market filed in the date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn ov_lst_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<OverseasListingDecision>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/ovLstDecsn.json");
        let rows: Vec<OverseasListingDecision> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20180118000015");
        assert_eq!(
            rows[0].lstprstk_ostk_cnt.as_ref().map(Number::as_str),
            Some("8,261,731")
        );
    }
}
