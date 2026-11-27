//! 부도발생 (`dfOcr.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "dfOcr.json";

/// One 부도발생 report: dishonored bills and defaults, returned by
/// [`Client::df_ocr`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct DefaultEvent {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 부도내용
    #[serde(default)]
    pub df_cn: String,
    /// 부도금액
    #[serde(default, deserialize_with = "decode::number")]
    pub df_amt: Option<Number>,
    /// 부도발생은행
    #[serde(default)]
    pub df_bnk: String,
    /// 최종부도(당좌거래정지)일자
    #[serde(default)]
    pub dfd: String,
    /// 부도사유 및 경위
    #[serde(default)]
    pub df_rs: String,
}

impl Client {
    /// 부도발생 (`dfOcr.json`): dishonored bills and defaults filed in the date
    /// range.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn df_ocr(&self, request: &DateRangeRequest) -> Result<Vec<DefaultEvent>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/dfOcr.json");
        let rows: Vec<DefaultEvent> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20191107000327");
        assert_eq!(
            rows[0].df_amt.as_ref().map(Number::as_str),
            Some("440,558,786")
        );
    }
}
