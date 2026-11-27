//! 해외 증권시장 주권등 상장폐지 결정 (`ovDlstDecsn.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "ovDlstDecsn.json";

/// One 해외 증권시장 주권등 상장폐지 결정 report: decisions to delist shares from an overseas
/// market, returned by [`Client::ov_dlst_decsn`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct OverseasDelistingDecision {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 상장폐지주식 종류ㆍ수(주)(보통주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub dlststk_ostk_cnt: Option<Number>,
    /// 상장폐지주식 종류ㆍ수(주)(기타주식)
    #[serde(default, deserialize_with = "decode::number")]
    pub dlststk_estk_cnt: Option<Number>,
    /// 상장거래소(소재국가)
    #[serde(default)]
    pub lstex_nt: String,
    /// 폐지신청예정일자
    #[serde(default)]
    pub dlstrq_prd: String,
    /// 폐지(예정)일자
    #[serde(default)]
    pub dlst_prd: String,
    /// 폐지사유
    #[serde(default)]
    pub dlst_rs: String,
    /// 이사회결의일(확인일)
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
    /// 해외 증권시장 주권등 상장폐지 결정 (`ovDlstDecsn.json`): decisions to delist shares
    /// from an overseas market filed in the date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn ov_dlst_decsn(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<OverseasDelistingDecision>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/ovDlstDecsn.json");
        let rows: Vec<OverseasDelistingDecision> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20191030000356");
        assert_eq!(rows[0].od_a_at_t.as_ref().map(Number::as_str), Some("4"));
    }
}
