//! 기업개황 (`company.json`).

use jiff::civil::Date;
use serde::{Deserialize, Deserializer};

use crate::client::Query;
use crate::{Client, CorpClass, Error, decode};

const ENDPOINT: &str = "company.json";

/// Company overview returned by [`Client::company`].
///
/// Text fields are passed through as OpenDART sends them and may be empty;
/// some companies put placeholder text such as `"없음"` in `hm_url`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct Company {
    /// 고유번호: the 8-digit OpenDART code.
    pub corp_code: String,
    /// 정식명칭: the official name, e.g. `"삼성전자(주)"`.
    pub corp_name: String,
    /// 영문명칭: the official English name.
    pub corp_name_eng: String,
    /// 종목명 for listed companies, 약식명칭 otherwise.
    pub stock_name: String,
    /// 종목코드: the 6-character stock code, `None` for unlisted companies.
    #[serde(default, deserialize_with = "decode::opt_string")]
    pub stock_code: Option<String>,
    /// 대표자명: the CEO name(s).
    pub ceo_nm: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 법인등록번호: corporate registration number.
    pub jurir_no: String,
    /// 사업자등록번호: business registration number.
    pub bizr_no: String,
    /// 주소: address.
    pub adres: String,
    /// 홈페이지: website.
    pub hm_url: String,
    /// IR홈페이지: investor relations website.
    pub ir_url: String,
    /// 전화번호: phone number.
    pub phn_no: String,
    /// 팩스번호: fax number.
    pub fax_no: String,
    /// 업종코드: the KSIC industry code, e.g. `"264"`.
    pub induty_code: String,
    /// 설립일: date of establishment.
    #[serde(default, deserialize_with = "decode::opt_date")]
    pub est_dt: Option<Date>,
    /// 결산월: the month the fiscal year ends, 1 to 12.
    #[serde(default, deserialize_with = "opt_month")]
    pub acc_mt: Option<u8>,
}

fn opt_month<'de, D: Deserializer<'de>>(de: D) -> Result<Option<u8>, D::Error> {
    let Some(text) = decode::opt_string(de)? else {
        return Ok(None);
    };
    match text.parse() {
        Ok(month @ 1..=12) => Ok(Some(month)),
        _ => Err(serde::de::Error::custom(format!(
            "expected a month (MM), got {text:?}"
        ))),
    }
}

impl Client {
    /// 기업개황: company overview (`company.json`).
    ///
    /// An unknown `corp_code` fails with [`Status::NoData`](crate::Status::NoData).
    pub async fn company(&self, corp_code: &str) -> Result<Company, Error> {
        let body = self.get(ENDPOINT, &query(corp_code)).await?;
        decode::json(ENDPOINT, &body)
    }
}

fn query(corp_code: &str) -> Query {
    vec![("corp_code", corp_code.to_owned())]
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn request_matches_the_guide() {
        crate::guide_check::assert_documented(ENDPOINT, &query("00126380"));
    }

    #[test]
    fn parses_a_listed_company() {
        let body = include_bytes!("../../tests/fixtures/company/listed.json");
        let company: Company = decode::json(ENDPOINT, body).unwrap();
        assert_eq!(company.corp_name, "삼성전자(주)");
        assert_eq!(company.stock_code.as_deref(), Some("005930"));
        assert_eq!(company.corp_cls, CorpClass::Kospi);
        assert_eq!(company.induty_code, "264");
        assert_eq!(company.est_dt, Some(date(1969, 1, 13)));
        assert_eq!(company.acc_mt, Some(12));
        assert_eq!(company.ir_url, "");
    }

    #[test]
    fn parses_an_unlisted_company() {
        let body = include_bytes!("../../tests/fixtures/company/unlisted.json");
        let company: Company = decode::json(ENDPOINT, body).unwrap();
        assert_eq!(company.stock_code, None);
        assert_eq!(company.corp_cls, CorpClass::Etc);
        assert_eq!(company.hm_url, "없음");
    }

    #[test]
    fn missing_optional_keys_are_none() {
        let body = include_bytes!("../../tests/fixtures/company/listed.json");
        let mut value: serde_json::Value = serde_json::from_slice(body).unwrap();
        for key in ["stock_code", "est_dt", "acc_mt"] {
            value.as_object_mut().unwrap().remove(key);
        }
        let company: Company = decode::json(ENDPOINT, value.to_string().as_bytes()).unwrap();
        assert_eq!(company.stock_code, None);
        assert_eq!(company.est_dt, None);
        assert_eq!(company.acc_mt, None);
    }
}
