//! 단일회사 전체 재무제표 (`fnlttSinglAcntAll.json`).

use serde::Deserialize;

use crate::client::Query;
use crate::{Client, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "fnlttSinglAcntAll.json";

/// Separate or consolidated statements (`fs_div`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Consolidation {
    /// `CFS`: 연결재무제표 (consolidated).
    Consolidated,
    /// `OFS`: 재무제표 (separate).
    Separate,
}

impl Consolidation {
    /// The raw code, `"CFS"` or `"OFS"`.
    pub fn code(self) -> &'static str {
        match self {
            Consolidation::Consolidated => "CFS",
            Consolidation::Separate => "OFS",
        }
    }
}

/// One line of a full financial statement, returned by
/// [`Client::fnltt_singl_acnt_all`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct StatementLine {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 보고서 코드, e.g. `11011`.
    #[serde(default)]
    pub reprt_code: String,
    /// 사업연도
    #[serde(default)]
    pub bsns_year: String,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 재무제표구분: `BS`, `IS`, `CIS`, `CF` or `SCE`.
    #[serde(default)]
    pub sj_div: String,
    /// 재무제표명
    #[serde(default)]
    pub sj_nm: String,
    /// 계정ID: the XBRL element, e.g. `ifrs-full_Assets`.
    #[serde(default)]
    pub account_id: String,
    /// 계정명
    #[serde(default)]
    pub account_nm: String,
    /// 계정상세: in the statement of changes in equity, the `|`-separated
    /// equity components; `-` in the other statements.
    #[serde(default)]
    pub account_detail: String,
    /// 당기명
    #[serde(default)]
    pub thstrm_nm: String,
    /// 당기금액
    #[serde(default, deserialize_with = "decode::number")]
    pub thstrm_amount: Option<Number>,
    /// 당기누적금액
    #[serde(default, deserialize_with = "decode::number")]
    pub thstrm_add_amount: Option<Number>,
    /// 전기명
    #[serde(default)]
    pub frmtrm_nm: String,
    /// 전기금액
    #[serde(default, deserialize_with = "decode::number")]
    pub frmtrm_amount: Option<Number>,
    /// 전기명(분/반기)
    #[serde(default)]
    pub frmtrm_q_nm: String,
    /// 전기금액(분/반기): the three-month amount in quarterly and half-year income statements.
    #[serde(default, deserialize_with = "decode::number")]
    pub frmtrm_q_amount: Option<Number>,
    /// 전기누적금액
    #[serde(default, deserialize_with = "decode::number")]
    pub frmtrm_add_amount: Option<Number>,
    /// 전전기명: in annual reports only.
    #[serde(default)]
    pub bfefrmtrm_nm: String,
    /// 전전기금액: in annual reports only.
    #[serde(default, deserialize_with = "decode::number")]
    pub bfefrmtrm_amount: Option<Number>,
    /// 계정과목 정렬순서
    #[serde(default, deserialize_with = "decode::number")]
    pub ord: Option<Number>,
    /// 통화 단위, e.g. `KRW`.
    #[serde(default)]
    pub currency: String,
}

impl Client {
    /// 단일회사 전체 재무제표 (`fnlttSinglAcntAll.json`): every line of the
    /// balance sheet, income statements, cash flow statement and statement
    /// of changes in equity in one company's report, about 200 to 300 rows.
    ///
    /// Returns an empty list when the report has no such statements
    /// ([`Status::NoData`](crate::Status::NoData)), for example
    /// [`Consolidation::Consolidated`] for a company without subsidiaries.
    pub async fn fnltt_singl_acnt_all(
        &self,
        request: &ReportRequest,
        fs_div: Consolidation,
    ) -> Result<Vec<StatementLine>, Error> {
        self.rows(ENDPOINT, &query(request, fs_div)).await
    }
}

fn query(request: &ReportRequest, fs_div: Consolidation) -> Query {
    let mut query = request.query();
    query.push(("fs_div", fs_div.code().to_owned()));
    query
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_matches_the_guide() {
        let request = ReportRequest::new("00126380", 2025, crate::ReportCode::Annual);
        crate::guide_check::assert_documented(
            ENDPOINT,
            &query(&request, Consolidation::Consolidated),
        );
    }

    #[test]
    fn decodes_fixture() {
        let body = include_bytes!("../../tests/fixtures/financial/fnlttSinglAcntAll.json");
        let rows: Vec<StatementLine> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 5);
        assert_eq!(rows[0].account_id, "ifrs-full_Assets");
        // Amounts in this endpoint come without thousands separators.
        assert_eq!(
            rows[0].thstrm_amount.as_ref().unwrap().as_str(),
            "566942110000000"
        );
        assert!(rows.iter().any(|row| row.thstrm_add_amount.is_none()));
    }
}
