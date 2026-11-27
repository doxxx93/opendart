//! 재무제표 원본파일(XBRL) (`fnlttXbrl.xml`).

use crate::client::Query;
use crate::disclosure::{DocumentFile, zip_files};
use crate::{Client, Error, ReportCode};

const ENDPOINT: &str = "fnlttXbrl.xml";

impl Client {
    /// 재무제표 원본파일(XBRL) (`fnlttXbrl.xml`): the XBRL instance and
    /// taxonomy files of one report's financial statements.
    ///
    /// `rcept_no` is the 14-digit receipt number of the report, e.g.
    /// [`Account::rcept_no`](crate::financial::Account::rcept_no). The
    /// download is a ZIP of about 1 MB for a large company (20 MB
    /// unpacked). An unknown receipt number fails with
    /// [`Status::NoData`](crate::Status::NoData).
    pub async fn fnltt_xbrl(
        &self,
        rcept_no: &str,
        reprt_code: ReportCode,
    ) -> Result<Vec<DocumentFile>, Error> {
        let body = self.get(ENDPOINT, &query(rcept_no, reprt_code)).await?;
        zip_files(ENDPOINT, &body)
    }
}

fn query(rcept_no: &str, reprt_code: ReportCode) -> Query {
    vec![
        ("rcept_no", rcept_no.to_owned()),
        ("reprt_code", reprt_code.code().to_owned()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_matches_the_guide() {
        crate::guide_check::assert_documented(
            ENDPOINT,
            &query("20260310002820", ReportCode::Annual),
        );
    }

    /// The fixture keeps the real archive's entries, each cut to 256 bytes.
    #[test]
    fn reads_every_file() {
        let body = include_bytes!("../../tests/fixtures/financial/fnlttXbrl.zip");
        let files = zip_files(ENDPOINT, body).unwrap();
        let names: Vec<&str> = files.iter().map(|file| file.name.as_str()).collect();
        assert_eq!(
            names,
            [
                "entity00126380_2025-12-31.xbrl",
                "entity00126380_2025-12-31.xsd",
                "entity00126380_2025-12-31_def.xml",
                "entity00126380_2025-12-31_cal.xml",
                "entity00126380_2025-12-31_pre.xml",
                "entity00126380_2025-12-31_lab-ko.xml",
                "entity00126380_2025-12-31_lab-en.xml",
            ]
        );
        assert!(files[0].content.starts_with(b"<?xml"));
    }
}
