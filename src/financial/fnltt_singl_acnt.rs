//! 단일회사 주요계정 (`fnlttSinglAcnt.json`).

use serde::Deserialize;

use crate::{Client, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "fnlttSinglAcnt.json";

/// One key account from a company's balance sheet or income statement,
/// returned by [`Client::fnltt_singl_acnt`] and [`Client::fnltt_multi_acnt`].
///
/// Each account appears twice, once from the separate (`OFS`) and once from
/// the consolidated (`CFS`) statements, when the company reports both.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct Account {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 사업연도
    #[serde(default)]
    pub bsns_year: String,
    /// 고유번호: the company's 8-digit OpenDART code. The replies carry it
    /// although the guide does not list it.
    #[serde(default)]
    pub corp_code: String,
    /// 종목코드
    #[serde(default, deserialize_with = "decode::opt_string")]
    pub stock_code: Option<String>,
    /// 보고서 코드, e.g. `11011`.
    #[serde(default)]
    pub reprt_code: String,
    /// 계정명, e.g. `자산총계`.
    #[serde(default)]
    pub account_nm: String,
    /// 개별/연결구분: `OFS` (재무제표) or `CFS` (연결재무제표).
    #[serde(default)]
    pub fs_div: String,
    /// 개별/연결명
    #[serde(default)]
    pub fs_nm: String,
    /// 재무제표구분: `BS` (재무상태표) or `IS` (손익계산서).
    #[serde(default)]
    pub sj_div: String,
    /// 재무제표명
    #[serde(default)]
    pub sj_nm: String,
    /// 당기명, e.g. `제 57 기`.
    #[serde(default)]
    pub thstrm_nm: String,
    /// 당기일자, e.g. `2025.12.31 현재`.
    #[serde(default)]
    pub thstrm_dt: String,
    /// 당기금액
    #[serde(default, deserialize_with = "decode::number")]
    pub thstrm_amount: Option<Number>,
    /// 당기누적금액
    #[serde(default, deserialize_with = "decode::number")]
    pub thstrm_add_amount: Option<Number>,
    /// 전기명
    #[serde(default)]
    pub frmtrm_nm: String,
    /// 전기일자
    #[serde(default)]
    pub frmtrm_dt: String,
    /// 전기금액
    #[serde(default, deserialize_with = "decode::number")]
    pub frmtrm_amount: Option<Number>,
    /// 전기누적금액
    #[serde(default, deserialize_with = "decode::number")]
    pub frmtrm_add_amount: Option<Number>,
    /// 전전기명: in annual reports only.
    #[serde(default)]
    pub bfefrmtrm_nm: String,
    /// 전전기일자: in annual reports only.
    #[serde(default)]
    pub bfefrmtrm_dt: String,
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
    /// 단일회사 주요계정 (`fnlttSinglAcnt.json`): key accounts (assets,
    /// liabilities, equity, revenue, operating income, net income) from one
    /// company's report.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn fnltt_singl_acnt(&self, request: &ReportRequest) -> Result<Vec<Account>, Error> {
        self.report_rows(ENDPOINT, request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_matches_the_guide() {
        let request = ReportRequest::new("00126380", 2025, crate::ReportCode::Annual);
        crate::guide_check::assert_documented(ENDPOINT, &request.query());
    }

    #[test]
    fn decodes_fixture() {
        let body = include_bytes!("../../tests/fixtures/financial/fnlttSinglAcnt.json");
        let rows: Vec<Account> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].account_nm, "유동자산");
        assert_eq!(rows[0].stock_code.as_deref(), Some("005930"));
        let amount: i64 = rows[0].thstrm_amount.as_ref().unwrap().parse().unwrap();
        assert_eq!(amount, 247_684_612_000_000);
    }
}
