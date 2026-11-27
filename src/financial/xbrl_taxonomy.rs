//! XBRL택사노미재무제표양식 (`xbrlTaxonomy.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::client::Query;
use crate::{Client, Error, decode};

const ENDPOINT: &str = "xbrlTaxonomy.json";

/// One account of a standard statement form, returned by
/// [`Client::xbrl_taxonomy`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct TaxonomyAccount {
    /// 재무제표구분, e.g. `BS1`.
    #[serde(default)]
    pub sj_div: String,
    /// 계정ID, e.g. `ifrs_CurrentAssets`.
    #[serde(default)]
    pub account_id: String,
    /// 계정명
    #[serde(default)]
    pub account_nm: String,
    /// 적용 기준일
    #[serde(default, deserialize_with = "decode::opt_date")]
    pub bsns_de: Option<Date>,
    /// 한글 출력명
    #[serde(default)]
    pub label_kor: String,
    /// 영문 출력명
    #[serde(default)]
    pub label_eng: String,
    /// 데이터 유형: `text block` (제목), `Text`, `yyyy-mm-dd` (date), `X`
    /// (amount), `(X)` (negative amount), `X.XX` (decimal), `Shares` (주식
    /// 수) or `For each` (repeated items). Empty, or absent from the reply,
    /// when no value is entered.
    #[serde(default)]
    pub data_tp: String,
    /// IFRS Reference
    #[serde(default)]
    pub ifrs_ref: String,
}

impl Client {
    /// XBRL택사노미재무제표양식 (`xbrlTaxonomy.json`): the accounts of one
    /// standard statement form of the DART XBRL taxonomy.
    ///
    /// `sj_div` names the form: `BS1` to `BS4` (재무상태표), `IS1` to `IS4`
    /// (손익계산서), `CIS1` to `CIS4` (포괄손익계산서), `DCIS1` to `DCIS8`
    /// (단일 포괄손익계산서), `CF1` to `CF4` (현금흐름표) and `SCE1`, `SCE2`
    /// (자본변동표). Odd numbers are consolidated, even numbers separate; the
    /// [guide](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS003&apiId=2020001)
    /// lists what else each number means.
    pub async fn xbrl_taxonomy(&self, sj_div: &str) -> Result<Vec<TaxonomyAccount>, Error> {
        self.rows(ENDPOINT, &query(sj_div)).await
    }
}

fn query(sj_div: &str) -> Query {
    vec![("sj_div", sj_div.to_owned())]
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn request_matches_the_guide() {
        crate::guide_check::assert_documented(ENDPOINT, &query("BS1"));
    }

    #[test]
    fn decodes_fixture() {
        let body = include_bytes!("../../tests/fixtures/financial/xbrlTaxonomy.json");
        let rows: Vec<TaxonomyAccount> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 5);
        assert_eq!(rows[0].label_kor, "재무상태표 [abstract]");
        assert_eq!(rows[0].bsns_de, Some(date(2018, 7, 1)));
        assert!(rows.iter().any(|row| row.data_tp.is_empty()));
    }
}
