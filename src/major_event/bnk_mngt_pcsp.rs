//! 채권은행 등의 관리절차 중단 (`bnkMngtPcsp.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error};

const ENDPOINT: &str = "bnkMngtPcsp.json";

/// One 채권은행 등의 관리절차 중단 report: the end of management by creditor banks,
/// returned by [`Client::bnk_mngt_pcsp`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct CreditorManagementEnd {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 관리절차중단 결정일자
    #[serde(default)]
    pub mngt_pcsp_dd: String,
    /// 관리기관
    #[serde(default)]
    pub mngt_int: String,
    /// 중단사유
    #[serde(default)]
    pub sp_rs: String,
    /// 향후대책
    #[serde(default)]
    pub ft_ctp: String,
    /// 확인일자
    #[serde(default)]
    pub cfd: String,
}

impl Client {
    /// 채권은행 등의 관리절차 중단 (`bnkMngtPcsp.json`): the end of management by creditor
    /// banks filed in the date range.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn bnk_mngt_pcsp(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<CreditorManagementEnd>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/bnkMngtPcsp.json");
        let rows: Vec<CreditorManagementEnd> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20160229001477");
    }
}
