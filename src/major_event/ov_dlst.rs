//! 해외 증권시장 주권등 상장폐지 (`ovDlst.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "ovDlst.json";

/// One 해외 증권시장 주권등 상장폐지 report: delistings of shares from an overseas market,
/// returned by [`Client::ov_dlst`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct OverseasDelisting {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 상장거래소 및 소재국가
    #[serde(default)]
    pub lstex_nt: String,
    /// 상장폐지주식의 종류(보통주식(주))
    #[serde(default, deserialize_with = "decode::number")]
    pub dlststk_ostk_cnt: Option<Number>,
    /// 상장폐지주식의 종류(기타주식(주))
    #[serde(default, deserialize_with = "decode::number")]
    pub dlststk_estk_cnt: Option<Number>,
    /// 매매거래종료일
    #[serde(default)]
    pub tredd: String,
    /// 폐지사유
    #[serde(default)]
    pub dlst_rs: String,
    /// 확인일자
    #[serde(default)]
    pub cfd: String,
}

impl Client {
    /// 해외 증권시장 주권등 상장폐지 (`ovDlst.json`): delistings of shares from an overseas
    /// market filed in the date range.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn ov_dlst(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<OverseasDelisting>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/ovDlst.json");
        let rows: Vec<OverseasDelisting> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20191205000311");
    }
}
