//! 소송 등의 제기 (`lwstLg.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error};

const ENDPOINT: &str = "lwstLg.json";

/// One 소송 등의 제기 report: lawsuits, returned by [`Client::lwst_lg`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct Lawsuit {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 사건의 명칭
    #[serde(default)]
    pub icnm: String,
    /// 원고ㆍ신청인
    #[serde(default)]
    pub ac_ap: String,
    /// 청구내용
    #[serde(default)]
    pub rq_cn: String,
    /// 관할법원
    #[serde(default)]
    pub cpct: String,
    /// 향후대책
    #[serde(default)]
    pub ft_ctp: String,
    /// 제기일자
    #[serde(default)]
    pub lgd: String,
    /// 확인일자
    #[serde(default)]
    pub cfd: String,
}

impl Client {
    /// 소송 등의 제기 (`lwstLg.json`): lawsuits filed in the date range.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn lwst_lg(&self, request: &DateRangeRequest) -> Result<Vec<Lawsuit>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/lwstLg.json");
        let rows: Vec<Lawsuit> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].rcept_no, "20190830001368");
    }
}
