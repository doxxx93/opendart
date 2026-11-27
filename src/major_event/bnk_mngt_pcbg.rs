//! 채권은행 등의 관리절차 개시 (`bnkMngtPcbg.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error};

const ENDPOINT: &str = "bnkMngtPcbg.json";

/// One 채권은행 등의 관리절차 개시 report: the start of management by creditor banks,
/// returned by [`Client::bnk_mngt_pcbg`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct CreditorManagementStart {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 관리절차개시 결정일자
    #[serde(default)]
    pub mngt_pcbg_dd: String,
    /// 관리기관
    #[serde(default)]
    pub mngt_int: String,
    /// 관리기간
    #[serde(default)]
    pub mngt_pd: String,
    /// 관리사유
    #[serde(default)]
    pub mngt_rs: String,
    /// 확인일자
    #[serde(default)]
    pub cfd: String,
}

impl Client {
    /// 채권은행 등의 관리절차 개시 (`bnkMngtPcbg.json`): the start of management by
    /// creditor banks filed in the date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn bnk_mngt_pcbg(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<CreditorManagementStart>, Error> {
        self.date_range_rows(ENDPOINT, request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode;

    #[test]
    fn request_matches_the_guide() {
        crate::guide_check::assert_documented(ENDPOINT, &crate::date_range::example().query());
    }

    #[test]
    fn decodes_fixture() {
        let body = include_bytes!("../../tests/fixtures/major_event/bnkMngtPcbg.json");
        let rows: Vec<CreditorManagementStart> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20191010000630");
    }
}
