//! 공시서류원본파일 (`document.xml`).

use std::io::Read;

use crate::client::Query;
use crate::{Client, Error, decode};

const ENDPOINT: &str = "document.xml";

/// One file from a filing's original documents, returned by
/// [`Client::document`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct DocumentFile {
    /// The file name inside the ZIP, e.g. `"20261126800680.xml"`, as the
    /// archive stores it. It comes from the server, so sanitize it before
    /// using it as a path.
    pub name: String,
    /// The raw bytes. Documents are DART XML or HTML; do not trust the
    /// declared charset, since some files declare `euc-kr` but are UTF-8.
    pub content: Vec<u8>,
}

impl Client {
    /// 공시서류원본파일: the original documents of one filing
    /// (`document.xml`).
    ///
    /// `rcept_no` is the 14-digit receipt number, e.g.
    /// [`Disclosure::rcept_no`](crate::disclosure::Disclosure::rcept_no).
    /// An unknown receipt number fails with
    /// [`Status::NoData`](crate::Status::NoData).
    pub async fn document(&self, rcept_no: &str) -> Result<Vec<DocumentFile>, Error> {
        let body = self.get(ENDPOINT, &query(rcept_no)).await?;
        parse(&body)
    }
}

fn query(rcept_no: &str) -> Query {
    vec![("rcept_no", rcept_no.to_owned())]
}

fn parse(body: &[u8]) -> Result<Vec<DocumentFile>, Error> {
    let mut archive = decode::zip(ENDPOINT, body)?;
    let mut files = Vec::with_capacity(archive.len());
    for index in 0..archive.len() {
        let mut file = archive
            .by_index(index)
            .map_err(|err| Error::decode(ENDPOINT, err))?;
        if file.is_dir() {
            continue;
        }
        let name = file
            .name()
            .map_err(|err| Error::decode(ENDPOINT, err))?
            .into_owned();
        let mut content = Vec::new();
        file.read_to_end(&mut content)
            .map_err(|err| Error::decode(ENDPOINT, err))?;
        files.push(DocumentFile { name, content });
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Status;

    #[test]
    fn unpacks_the_real_zip() {
        let body = include_bytes!("../../tests/fixtures/document/20261126800680.zip");
        let files = parse(body).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "20261126800680.xml");
        assert_eq!(files[0].content.len(), 7964);
        assert!(files[0].content.starts_with(b"<html>"));
    }

    #[test]
    fn request_matches_the_guide() {
        crate::guide_check::assert_documented(ENDPOINT, &query("20261126800680"));
    }

    #[test]
    fn unknown_receipt_number() {
        let body = include_bytes!("../../tests/fixtures/document/not_found.xml");
        match parse(body).unwrap_err() {
            Error::Api {
                status, message, ..
            } => {
                assert_eq!(status, Status::NoData);
                assert_eq!(message, "접수번호 오류 : 20261126999999");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
