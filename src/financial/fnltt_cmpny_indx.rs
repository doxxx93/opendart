//! 다중회사 주요 재무지표 (`fnlttCmpnyIndx.json`).

use super::{Indicator, IndicatorClass};
use crate::{Client, Error, MultiReportRequest};

const ENDPOINT: &str = "fnlttCmpnyIndx.json";

impl Client {
    /// 다중회사 주요 재무지표 (`fnlttCmpnyIndx.json`): the indicators of
    /// [`fnltt_singl_indx`](Client::fnltt_singl_indx) for up to 100
    /// companies at once. Tell the companies apart by
    /// [`Indicator::corp_code`].
    ///
    /// Returns an empty list when there are no indicators for any of the
    /// reports ([`Status::NoData`](crate::Status::NoData)).
    pub async fn fnltt_cmpny_indx(
        &self,
        request: &MultiReportRequest,
        idx_cl_code: IndicatorClass,
    ) -> Result<Vec<Indicator>, Error> {
        self.rows(
            ENDPOINT,
            &super::fnltt_singl_indx::query(request.query(), idx_cl_code),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode;

    #[test]
    fn request_matches_the_guide() {
        let request =
            MultiReportRequest::new(["00126380", "00164779"], 2025, crate::ReportCode::Annual);
        let query =
            super::super::fnltt_singl_indx::query(request.query(), IndicatorClass::Stability);
        crate::guide_check::assert_documented(ENDPOINT, &query);
    }

    #[test]
    fn decodes_fixture() {
        let body = include_bytes!("../../tests/fixtures/financial/fnlttCmpnyIndx.json");
        let rows: Vec<Indicator> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 6);
        assert!(rows.iter().any(|row| row.corp_code == "00164779"));
    }
}
