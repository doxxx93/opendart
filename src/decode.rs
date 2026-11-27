//! Response decoding shared by the endpoints: the status envelope, ZIP
//! bodies with XML error replies, and OpenDART's string-encoded values.

use std::io::Cursor;

use jiff::civil::Date;
use serde::de::{DeserializeOwned, Error as _};
use serde::{Deserialize, Deserializer};

use crate::error::{BoxError, Error, Status};

/// The `status` and `message` fields every OpenDART reply carries.
#[derive(Deserialize)]
struct Envelope {
    status: String,
    message: String,
}

impl Envelope {
    fn into_result(self) -> Result<(), Error> {
        match Status::from_code(&self.status) {
            None => Ok(()),
            Some(status) => Err(Error::Api {
                status,
                message: self.message,
            }),
        }
    }
}

/// Decodes a JSON body: first the status, then the payload.
///
/// Error replies carry only `status` and `message`, so the payload is
/// decoded only after the status says success.
pub(crate) fn json<T: DeserializeOwned>(endpoint: &'static str, body: &[u8]) -> Result<T, Error> {
    let envelope: Envelope =
        serde_json::from_slice(body).map_err(|err| Error::decode(endpoint, err))?;
    envelope.into_result()?;
    serde_json::from_slice(body).map_err(|err| Error::decode(endpoint, err))
}

/// Opens a ZIP body, or turns the XML error reply OpenDART sends instead
/// into an [`Error`].
pub(crate) fn zip<B: AsRef<[u8]>>(
    endpoint: &'static str,
    body: B,
) -> Result<zip::ZipArchive<Cursor<B>>, Error> {
    const ZIP_MAGIC: &[u8] = b"PK\x03\x04";
    if !body.as_ref().starts_with(ZIP_MAGIC) {
        return Err(xml_error(endpoint, body.as_ref()));
    }
    zip::ZipArchive::new(Cursor::new(body)).map_err(|err| Error::decode(endpoint, err))
}

fn xml_error(endpoint: &'static str, body: &[u8]) -> Error {
    let envelope = std::str::from_utf8(body)
        .map_err(BoxError::from)
        .and_then(|text| quick_xml::de::from_str::<Envelope>(text).map_err(BoxError::from));
    match envelope {
        Ok(envelope) => match envelope.into_result() {
            Err(err) => err,
            Ok(()) => Error::decode(endpoint, "status 000 but the body is not a ZIP file"),
        },
        Err(err) => Error::decode(endpoint, err),
    }
}

/// Parses a `YYYYMMDD` date.
pub(crate) fn parse_date(text: &str) -> Result<Date, BoxError> {
    let bytes = text.as_bytes();
    if bytes.len() != 8 || !bytes.iter().all(u8::is_ascii_digit) {
        return Err(format!("expected a YYYYMMDD date, got {text:?}").into());
    }
    let year = text[0..4].parse()?;
    let month = text[4..6].parse()?;
    let day = text[6..8].parse()?;
    Ok(Date::new(year, month, day)?)
}

/// Formats a date as `YYYYMMDD` for request parameters.
pub(crate) fn format_date(date: Date) -> String {
    format!("{:04}{:02}{:02}", date.year(), date.month(), date.day())
}

/// `deserialize_with` for a required `YYYYMMDD` field.
pub(crate) fn date<'de, D: Deserializer<'de>>(de: D) -> Result<Date, D::Error> {
    let text = String::deserialize(de)?;
    parse_date(&text).map_err(D::Error::custom)
}

/// `deserialize_with` for a `YYYYMMDD` field that may be blank.
pub(crate) fn opt_date<'de, D: Deserializer<'de>>(de: D) -> Result<Option<Date>, D::Error> {
    match opt_string(de)? {
        None => Ok(None),
        Some(text) => parse_date(&text).map(Some).map_err(D::Error::custom),
    }
}

/// `deserialize_with` for a string that OpenDART leaves blank (`""` or
/// `" "`) when there is no value, such as the stock code of an unlisted
/// company. Surrounding whitespace is trimmed.
pub(crate) fn opt_string<'de, D: Deserializer<'de>>(de: D) -> Result<Option<String>, D::Error> {
    let text = Option::<String>::deserialize(de)?.unwrap_or_default();
    let trimmed = text.trim();
    Ok((!trimmed.is_empty()).then(|| trimmed.to_owned()))
}

/// `deserialize_with` for a string with meaningless surrounding padding,
/// such as `report_nm` in `list.json`.
pub(crate) fn trimmed<'de, D: Deserializer<'de>>(de: D) -> Result<String, D::Error> {
    let text = String::deserialize(de)?;
    Ok(match text.trim() {
        trimmed if trimmed.len() == text.len() => text,
        trimmed => trimmed.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn dates_parse_and_format() {
        assert_eq!(parse_date("20261126").unwrap(), date(2026, 11, 26));
        assert_eq!(format_date(date(2026, 1, 5)), "20260105");
        for bad in ["", "2026112", "2026-11-26", "20261332", "abcdefgh"] {
            assert!(parse_date(bad).is_err(), "{bad:?} should not parse");
        }
    }

    #[test]
    fn json_error_status_becomes_api_error() {
        let body = include_bytes!("../tests/fixtures/error/unregistered_key.json");
        let err = json::<serde_json::Value>("list.json", body).unwrap_err();
        assert_eq!(err.status(), Some(&Status::UnregisteredKey));
    }

    #[test]
    fn xml_error_reply_instead_of_zip() {
        let body = include_bytes!("../tests/fixtures/error/unregistered_key.xml");
        let err = zip("corpCode.xml", &body[..]).unwrap_err();
        match err {
            Error::Api { status, message } => {
                assert_eq!(status, Status::UnregisteredKey);
                assert_eq!(message, "등록되지 않은 인증키입니다.");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
