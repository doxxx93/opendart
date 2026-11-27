//! 이사·감사 전체의 보수현황(보수지급금액 - 유형별) (`drctrAdtAllMendngSttusMendngPymntamtTyCl.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "drctrAdtAllMendngSttusMendngPymntamtTyCl.json";

/// One row of 이사·감사 전체의 보수현황(보수지급금액 - 유형별): pay of directors and auditors by
/// type.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct DirectorPayByType {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 법인명
    pub corp_name: String,
    /// 구분
    #[serde(default)]
    pub se: String,
    /// 인원수
    #[serde(default, deserialize_with = "decode::number")]
    pub nmpr: Option<Number>,
    /// 보수총액
    #[serde(default, deserialize_with = "decode::number")]
    pub pymnt_totamt: Option<Number>,
    /// 1인당 평균보수액
    #[serde(default, deserialize_with = "decode::number")]
    pub psn1_avrg_pymntamt: Option<Number>,
    /// 비고
    #[serde(default)]
    pub rm: String,
    /// 사업연도
    #[serde(default)]
    pub fscl_year: String,
    /// 보수총액 중 주식기준보상 지급액
    #[serde(default, deserialize_with = "decode::number")]
    pub stk_bsd_pd_mendng_totamt: Option<Number>,
    /// 주식매수선택권 행사가능수량
    #[serde(default, deserialize_with = "decode::number")]
    pub stk_opt_exrcsbl_qty: Option<Number>,
    /// 주식매수선택권 행사불가수량
    #[serde(default, deserialize_with = "decode::number")]
    pub stk_opt_unexrcsbl_qty: Option<Number>,
    /// 주식매수선택권 잔여금액
    #[serde(default, deserialize_with = "decode::number")]
    pub stk_opt_rmn_blce: Option<Number>,
    /// 그 외 주식기준 보상 미지급수량
    #[serde(default, deserialize_with = "decode::number")]
    pub othr_stk_bsd_cmpn_unpyd_qty: Option<Number>,
    /// 그 외 주식기준 보상 시장가치
    #[serde(default, deserialize_with = "decode::number")]
    pub othr_stk_bsd_cmpn_mkt_vl: Option<Number>,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 이사·감사 전체의 보수현황(보수지급금액 - 유형별)
    /// (`drctrAdtAllMendngSttusMendngPymntamtTyCl.json`): pay of directors and
    /// auditors by type.
    ///
    /// Returns an empty list when the report has no such section
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn drctr_adt_all_mendng_sttus_mendng_pymntamt_ty_cl(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<DirectorPayByType>, Error> {
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
        let body = include_bytes!(
            "../../tests/fixtures/periodic/drctrAdtAllMendngSttusMendngPymntamtTyCl.json"
        );
        let rows: Vec<DirectorPayByType> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].rcept_no, "20260310002820");
        assert_eq!(rows[0].nmpr.as_ref().map(Number::as_str), Some("3"));
    }
}
