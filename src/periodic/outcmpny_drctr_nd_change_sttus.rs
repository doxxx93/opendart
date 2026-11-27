//! 독립(사외)이사 및 그 변동현황 (`outcmpnyDrctrNdChangeSttus.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "outcmpnyDrctrNdChangeSttus.json";

/// One row of 독립(사외)이사 및 그 변동현황: outside directors and their changes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct OutsideDirectorCount {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 법인명
    pub corp_name: String,
    /// 이사의 수
    #[serde(default, deserialize_with = "decode::number")]
    pub drctr_co: Option<Number>,
    /// 독립(사외)이사 수
    #[serde(default, deserialize_with = "decode::number")]
    pub otcmp_drctr_co: Option<Number>,
    /// 독립(사외)이사 변동현황(선임)
    #[serde(default, deserialize_with = "decode::number")]
    pub apnt: Option<Number>,
    /// 독립(사외)이사 변동현황(해임)
    #[serde(default, deserialize_with = "decode::number")]
    pub rlsofc: Option<Number>,
    /// 독립(사외)이사 변동현황(중도퇴임)
    #[serde(default, deserialize_with = "decode::number")]
    pub mdstrm_resig: Option<Number>,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 독립(사외)이사 및 그 변동현황 (`outcmpnyDrctrNdChangeSttus.json`): outside directors
    /// and their changes.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn outcmpny_drctr_nd_change_sttus(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<OutsideDirectorCount>, Error> {
        self.report_rows(ENDPOINT, request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_matches_the_guide() {
        let request = ReportRequest::new("00126380", 2025, crate::ReportCode::Annual);
        crate::guide_check::assert_documented(ENDPOINT, &request.query());
    }

    #[test]
    fn decodes_fixture() {
        let body = include_bytes!("../../tests/fixtures/periodic/outcmpnyDrctrNdChangeSttus.json");
        let rows: Vec<OutsideDirectorCount> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(rows[0].drctr_co.as_ref().map(Number::as_str), Some("9"));
    }
}
