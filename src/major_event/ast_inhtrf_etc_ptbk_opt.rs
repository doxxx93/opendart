//! 자산양수도(기타), 풋백옵션 (`astInhtrfEtcPtbkOpt.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "astInhtrfEtcPtbkOpt.json";

/// One 자산양수도(기타), 풋백옵션 report: asset transfers (other) and put-back options,
/// returned by [`Client::ast_inhtrf_etc_ptbk_opt`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct AssetTransferPutBack {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 보고 사유
    #[serde(default)]
    pub rp_rsn: String,
    /// 자산양수ㆍ도 가액
    #[serde(default, deserialize_with = "decode::number")]
    pub ast_inhtrf_prc: Option<Number>,
}

impl Client {
    /// 자산양수도(기타), 풋백옵션 (`astInhtrfEtcPtbkOpt.json`): asset transfers (other)
    /// and put-back options filed in the date range.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn ast_inhtrf_etc_ptbk_opt(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<AssetTransferPutBack>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/astInhtrfEtcPtbkOpt.json");
        let rows: Vec<AssetTransferPutBack> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20190328001555");
        assert_eq!(
            rows[0].ast_inhtrf_prc.as_ref().map(Number::as_str),
            Some("130,410,000,000")
        );
    }
}
