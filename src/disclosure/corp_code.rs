//! 고유번호 (`corpCode.xml`).

use std::io::BufReader;

use jiff::civil::Date;
use serde::Deserialize;

use crate::client::Query;
use crate::{Client, Error, decode};

const ENDPOINT: &str = "corpCode.xml";

/// One company in the [`Client::corp_code`] listing.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct CorpCodeEntry {
    /// 고유번호: the 8-digit OpenDART code other endpoints take.
    pub corp_code: String,
    /// 정식명칭: the company name.
    pub corp_name: String,
    /// 영문 정식명칭: the English name, `None` when blank.
    #[serde(default, deserialize_with = "decode::opt_string")]
    pub corp_eng_name: Option<String>,
    /// 종목코드: the 6-character stock code, `None` for unlisted
    /// companies. Newer codes contain letters, e.g. `"0068Y0"`.
    #[serde(default, deserialize_with = "decode::opt_string")]
    pub stock_code: Option<String>,
    /// 최종변경일자: when the company overview last changed.
    #[serde(deserialize_with = "decode::date")]
    pub modify_date: Date,
}

#[derive(Deserialize)]
struct CorpCodeXml {
    #[serde(default)]
    list: Vec<CorpCodeEntry>,
}

impl Client {
    /// 고유번호: every company registered in DART (`corpCode.xml`).
    ///
    /// This maps stock codes to the `corp_code` values other endpoints
    /// take. The download is a ZIP of about 3.6 MB holding a 30 MB XML file
    /// with about 120,000 companies, of which about 4,000 are listed, so
    /// cache the result rather than calling this per request.
    pub async fn corp_code(&self) -> Result<Vec<CorpCodeEntry>, Error> {
        let body = self.get(ENDPOINT, &Query::new()).await?;
        parse(&body)
    }
}

fn parse(body: &[u8]) -> Result<Vec<CorpCodeEntry>, Error> {
    let mut archive = decode::zip(ENDPOINT, body)?;
    if archive.len() != 1 {
        return Err(Error::decode(
            ENDPOINT,
            format!("expected one file in the ZIP, found {}", archive.len()),
        ));
    }
    let file = archive
        .by_index(0)
        .map_err(|err| Error::decode(ENDPOINT, err))?;
    let xml: CorpCodeXml = quick_xml::de::from_reader(BufReader::new(file))
        .map_err(|err| Error::decode(ENDPOINT, err))?;
    Ok(xml.list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn request_matches_the_guide() {
        crate::guide_check::assert_documented(ENDPOINT, &Query::new());
    }

    #[test]
    fn parses_the_trimmed_real_zip() {
        let body = include_bytes!("../../tests/fixtures/corp_code/CORPCODE.zip");
        let entries = parse(body).unwrap();
        assert_eq!(entries.len(), 6);

        let daco = &entries[0];
        assert_eq!(daco.corp_code, "00434003");
        assert_eq!(daco.stock_code, None);
        assert_eq!(daco.modify_date, date(2017, 6, 30));
        assert_eq!(
            entries[1].corp_eng_name.as_deref(),
            Some("Good & LS Co.,Ltd.")
        );

        let samsung = &entries[2];
        assert_eq!(samsung.corp_name, "삼성전자");
        assert_eq!(samsung.stock_code.as_deref(), Some("005930"));

        let lettered = entries[3].stock_code.as_deref().unwrap();
        assert!(
            lettered.bytes().any(|b| b.is_ascii_uppercase()),
            "{lettered}"
        );
        assert_eq!(entries[4].corp_eng_name, None);
    }

    #[test]
    fn xml_error_reply() {
        let body = include_bytes!("../../tests/fixtures/error/unregistered_key.xml");
        let err = parse(body).unwrap_err();
        assert_eq!(err.status(), Some(&crate::Status::UnregisteredKey));
    }
}
