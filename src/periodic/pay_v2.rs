//! 이사·감사의 개인별 보수현황(5억원 이상) Ver 2.0
//! (`hmvAuditIndvdlBySttusV2.json`) and 개인별 보수지급 금액(5억이상 상위5인)
//! Ver 2.0 (`indvdlByPayV2.json`), added in 2026-05.
//!
//! Unlike the other periodic endpoints, these return one report with a
//! `group` per person, each with a `list` of stock-based pay.

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, Status, decode};

const INDIVIDUAL: &str = "hmvAuditIndvdlBySttusV2.json";
const TOP_FIVE: &str = "indvdlByPayV2.json";

/// Individual pay disclosed in one report, returned by
/// [`Client::hmv_audit_indvdl_by_sttus_v2`] and [`Client::indvdl_by_pay_v2`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct PayReport {
    /// 접수번호
    pub rcept_no: String,
    /// 법인구분
    pub corp_cls: CorpClass,
    /// 고유번호
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 결산기준일
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
    /// One entry per person and fiscal year.
    #[serde(default)]
    pub group: Vec<PayRecipient>,
}

/// One person's pay in a [`PayReport`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct PayRecipient {
    /// 이름
    #[serde(default)]
    pub nm: String,
    /// 사업연도: 당기, 전기 or 전전기.
    #[serde(default)]
    pub fscl_year: String,
    /// 직위
    #[serde(default)]
    pub ofcps: String,
    /// 보수 총액
    #[serde(default, deserialize_with = "decode::number")]
    pub mendng_totamt: Option<Number>,
    /// Stock-based pay included in the total.
    #[serde(default)]
    pub list: Vec<StockBasedPay>,
}

/// Stock-based pay of a [`PayRecipient`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct StockBasedPay {
    /// 보수총액 중 주식기준보상 지급액-종류
    #[serde(default)]
    pub stk_bsd_pd_mendng_totamt_knd: String,
    /// 보수총액 중 주식기준보상 지급액-수량
    #[serde(default, deserialize_with = "decode::number")]
    pub stk_bsd_pd_mendng_totamt_qty: Option<Number>,
    /// 보수총액 중 주식기준보상 지급액-금액
    #[serde(default, deserialize_with = "decode::number")]
    pub stk_bsd_pd_mendng_totamt_amt: Option<Number>,
    /// 주식매수선택권 행사가능수량
    #[serde(default, deserialize_with = "decode::number")]
    pub stk_opt_exrcsbl_qty: Option<Number>,
    /// 주식매수선택권 행사불가수량
    #[serde(default, deserialize_with = "decode::number")]
    pub stk_opt_unexrcsbl_qty: Option<Number>,
    /// 주식매수선택권 행사가격
    #[serde(default, deserialize_with = "decode::number")]
    pub stk_opt_exrc_pr: Option<Number>,
    /// 주식매수선택권 잔여금액
    #[serde(default, deserialize_with = "decode::number")]
    pub stk_opt_rmn_blce: Option<Number>,
    /// 그 외 주식기준 보상 미지급수량
    #[serde(default, deserialize_with = "decode::number")]
    pub othr_stk_bsd_cmpn_unpyd_qty: Option<Number>,
    /// 그 외 주식기준 보상 시장가치
    #[serde(default, deserialize_with = "decode::number")]
    pub othr_stk_bsd_cmpn_mkt_vl: Option<Number>,
    /// 비고
    #[serde(default)]
    pub rm: String,
}

impl Client {
    /// 이사·감사의 개인별 보수현황(5억원 이상) Ver 2.0
    /// (`hmvAuditIndvdlBySttusV2.json`): pay of each director and auditor
    /// paid 500 million won or more, with stock-based pay.
    ///
    /// Covers reports filed from 2026-05 on; earlier reports are in
    /// [`hmv_audit_indvdl_by_sttus`](Client::hmv_audit_indvdl_by_sttus).
    /// Returns `None` when the report has no such section
    /// ([`Status::NoData`]).
    pub async fn hmv_audit_indvdl_by_sttus_v2(
        &self,
        request: &ReportRequest,
    ) -> Result<Option<PayReport>, Error> {
        let body = self.get(INDIVIDUAL, &request.query()).await?;
        parse(INDIVIDUAL, &body)
    }

    /// 개인별 보수지급 금액(5억이상 상위5인) Ver 2.0 (`indvdlByPayV2.json`):
    /// the top five individuals paid 500 million won or more, with
    /// stock-based pay.
    ///
    /// Covers reports filed from 2026-05 on; earlier reports are in
    /// [`indvdl_by_pay`](Client::indvdl_by_pay). Returns `None` when the
    /// report has no such section ([`Status::NoData`]).
    pub async fn indvdl_by_pay_v2(
        &self,
        request: &ReportRequest,
    ) -> Result<Option<PayReport>, Error> {
        let body = self.get(TOP_FIVE, &request.query()).await?;
        parse(TOP_FIVE, &body)
    }
}

fn parse(endpoint: &'static str, body: &[u8]) -> Result<Option<PayReport>, Error> {
    match decode::json(endpoint, body) {
        Ok(report) => Ok(Some(report)),
        Err(Error::Api {
            status: Status::NoData,
            ..
        }) => Ok(None),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn decodes_individual_pay() {
        let body = include_bytes!("../../tests/fixtures/periodic/hmvAuditIndvdlBySttusV2.json");
        let report = parse(INDIVIDUAL, body).unwrap().unwrap();
        assert_eq!(report.corp_cls, CorpClass::Kospi);
        assert_eq!(report.stlm_dt, Some(date(2026, 6, 30)));
        assert_eq!(report.group.len(), 2);
        let person = &report.group[0];
        assert_eq!(person.fscl_year, "당기");
        assert_eq!(
            person.mendng_totamt.as_ref().unwrap().parse::<i64>(),
            Ok(2_393_000_000)
        );
        let stock = &person.list[0];
        assert_eq!(
            stock
                .stk_bsd_pd_mendng_totamt_qty
                .as_ref()
                .unwrap()
                .parse::<i64>(),
            Ok(5_135)
        );
        assert_eq!(stock.stk_opt_exrc_pr, None);
        // The second row has no `rm` key at all.
        assert_eq!(person.list[1].rm, "");
    }

    #[test]
    fn decodes_top_five() {
        let body = include_bytes!("../../tests/fixtures/periodic/indvdlByPayV2.json");
        let report = parse(TOP_FIVE, body).unwrap().unwrap();
        assert_eq!(report.group.len(), 2);
    }

    #[test]
    fn requests_match_the_guide() {
        let request = ReportRequest::new("00126380", 2026, crate::ReportCode::HalfYear);
        crate::guide_check::assert_documented(INDIVIDUAL, &request.query());
        crate::guide_check::assert_documented(TOP_FIVE, &request.query());
    }

    #[test]
    fn no_data_is_none() {
        let body = include_bytes!("../../tests/fixtures/list/no_data.json");
        assert_eq!(parse(TOP_FIVE, body).unwrap(), None);
    }
}
