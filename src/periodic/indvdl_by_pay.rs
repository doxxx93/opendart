//! 개인별 보수지급 금액(5억이상 상위5인) (`indvdlByPay.json`).

use super::hmv_audit_indvdl_by_sttus::IndividualPay;
use crate::{Client, Error, ReportRequest};

const ENDPOINT: &str = "indvdlByPay.json";

impl Client {
    /// 개인별 보수지급 금액(5억이상 상위5인) (`indvdlByPay.json`): top five individuals paid
    /// 500 million won or more.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn indvdl_by_pay(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<IndividualPay>, Error> {
        self.report_rows(ENDPOINT, request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Number, decode};

    #[test]
    fn request_matches_the_guide() {
        let request = ReportRequest::new("00126380", 2025, crate::ReportCode::Annual);
        crate::guide_check::assert_documented(ENDPOINT, &request.query());
    }

    #[test]
    fn decodes_fixture() {
        let body = include_bytes!("../../tests/fixtures/periodic/indvdlByPay.json");
        let rows: Vec<IndividualPay> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(
            rows[0].mendng_totamt.as_ref().map(Number::as_str),
            Some("13,407,000,000")
        );
    }
}
