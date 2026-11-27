//! 해외 증권시장 주권등 상장 (`ovLst.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "ovLst.json";

/// One 해외 증권시장 주권등 상장 report: listings of shares on an overseas market,
/// returned by [`Client::ov_lst`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct OverseasListing {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 상장주식 종류 및 수(보통주식(주))
    #[serde(default, deserialize_with = "decode::number")]
    pub lststk_ostk_cnt: Option<Number>,
    /// 상장주식 종류 및 수(기타주식(주))
    #[serde(default, deserialize_with = "decode::number")]
    pub lststk_estk_cnt: Option<Number>,
    /// 상장거래소(소재국가)
    #[serde(default)]
    pub lstex_nt: String,
    /// 종목 명 (code)
    #[serde(default)]
    pub stk_cd: String,
    /// 상장일자
    #[serde(default)]
    pub lstd: String,
    /// 확인일자
    #[serde(default)]
    pub cfd: String,
}

impl Client {
    /// 해외 증권시장 주권등 상장 (`ovLst.json`): listings of shares on an overseas market
    /// filed in the date range.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn ov_lst(&self, request: &DateRangeRequest) -> Result<Vec<OverseasListing>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/ovLst.json");
        let rows: Vec<OverseasListing> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20190214001013");
    }
}
