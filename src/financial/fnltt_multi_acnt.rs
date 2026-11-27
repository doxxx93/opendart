//! 다중회사 주요계정 (`fnlttMultiAcnt.json`).

use super::Account;
use crate::{Client, Error, MultiReportRequest};

const ENDPOINT: &str = "fnlttMultiAcnt.json";

impl Client {
    /// 다중회사 주요계정 (`fnlttMultiAcnt.json`): the key accounts of
    /// [`fnltt_singl_acnt`](Client::fnltt_singl_acnt) for up to 100
    /// companies at once. Tell the companies apart by
    /// [`Account::corp_code`].
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn fnltt_multi_acnt(
        &self,
        request: &MultiReportRequest,
    ) -> Result<Vec<Account>, Error> {
        self.rows(ENDPOINT, &request.query()).await
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
        crate::guide_check::assert_documented(ENDPOINT, &request.query());
    }

    #[test]
    fn decodes_fixture() {
        let body = include_bytes!("../../tests/fixtures/financial/fnlttMultiAcnt.json");
        let rows: Vec<Account> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 5);
        let mut companies: Vec<&str> = rows.iter().map(|row| row.corp_code.as_str()).collect();
        companies.dedup();
        assert_eq!(companies, ["00126380", "00164779"]);
    }
}
