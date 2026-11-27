//! 회생절차 개시신청 (`ctrcvsBgrq.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error};

const ENDPOINT: &str = "ctrcvsBgrq.json";

/// One 회생절차 개시신청 report: filings for rehabilitation proceedings, returned by
/// [`Client::ctrcvs_bgrq`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct RehabilitationFiling {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 신청인 (회사와의 관계)
    #[serde(default)]
    pub apcnt: String,
    /// 관할법원
    #[serde(default)]
    pub cpct: String,
    /// 신청사유
    #[serde(default)]
    pub rq_rs: String,
    /// 신청일자
    #[serde(default)]
    pub rqd: String,
    /// 향후대책 및 일정
    #[serde(default)]
    pub ft_ctp_sc: String,
}

impl Client {
    /// 회생절차 개시신청 (`ctrcvsBgrq.json`): filings for rehabilitation proceedings
    /// filed in the date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn ctrcvs_bgrq(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<RehabilitationFiling>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/ctrcvsBgrq.json");
        let rows: Vec<RehabilitationFiling> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].rcept_no, "20190418000228");
    }
}
