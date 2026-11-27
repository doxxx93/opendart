//! 해산사유 발생 (`dsRsOcr.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "dsRsOcr.json";

/// One 해산사유 발생 report: causes for dissolution, returned by
/// [`Client::ds_rs_ocr`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct DissolutionCause {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 해산사유
    #[serde(default)]
    pub ds_rs: String,
    /// 해산사유발생일(결정일)
    #[serde(default)]
    pub ds_rsd: String,
    /// 독립(사외)이사 참석여부(참석)
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_t: Option<Number>,
    /// 독립(사외)이사 참석여부(불참)
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_b: Option<Number>,
    /// 감사(감사위원) 참석 여부
    #[serde(default)]
    pub adt_a_atn: String,
}

impl Client {
    /// 해산사유 발생 (`dsRsOcr.json`): causes for dissolution filed in the date
    /// range.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn ds_rs_ocr(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<DissolutionCause>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/major_event/dsRsOcr.json");
        let rows: Vec<DissolutionCause> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20190131000116");
    }
}
