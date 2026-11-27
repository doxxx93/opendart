//! 공모자금의 사용내역 (`pssrpCptalUseDtls.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, CorpClass, Error, Number, ReportRequest, decode};

const ENDPOINT: &str = "pssrpCptalUseDtls.json";

/// One row of 공모자금의 사용내역: use of funds raised by public offerings.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct PublicOfferingFundUse {
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
    pub se_nm: String,
    /// 회차: ③ 2019년 12월 9일부터 추가됨
    #[serde(default)]
    pub tm: String,
    /// 납입일
    #[serde(default)]
    pub pay_de: String,
    /// 납입금액: ① 2018년 1월 18일까지 사용됨
    #[serde(default, deserialize_with = "decode::number")]
    pub pay_amount: Option<Number>,
    /// 신고서상 자금사용 계획: ① 2018년 1월 18일까지 사용됨
    #[serde(default)]
    pub on_dclrt_cptal_use_plan: String,
    /// 실제 자금사용 현황: ① 2018년 1월 18일까지 사용됨
    #[serde(default)]
    pub real_cptal_use_sttus: String,
    /// 증권신고서 등의 자금사용 계획(사용용도): ② 2018년 1월 19일부터 추가됨
    #[serde(default)]
    pub rs_cptal_use_plan_useprps: String,
    /// 증권신고서 등의 자금사용 계획(조달금액): ② 2018년 1월 19일부터 추가됨
    #[serde(default, deserialize_with = "decode::number")]
    pub rs_cptal_use_plan_prcure_amount: Option<Number>,
    /// 실제 자금사용 내역(내용): ② 2018년 1월 19일부터 추가됨
    #[serde(default)]
    pub real_cptal_use_dtls_cn: String,
    /// 실제 자금사용 내역(금액): ② 2018년 1월 19일부터 추가됨
    #[serde(default, deserialize_with = "decode::number")]
    pub real_cptal_use_dtls_amount: Option<Number>,
    /// 차이발생 사유 등
    #[serde(default)]
    pub dffrnc_occrrnc_resn: String,
    /// 결산기준일: the end of the reporting period.
    #[serde(default, deserialize_with = "decode::opt_iso_date")]
    pub stlm_dt: Option<Date>,
}

impl Client {
    /// 공모자금의 사용내역 (`pssrpCptalUseDtls.json`): use of funds raised by public
    /// offerings.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn pssrp_cptal_use_dtls(
        &self,
        request: &ReportRequest,
    ) -> Result<Vec<PublicOfferingFundUse>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/periodic/pssrpCptalUseDtls.json");
        let rows: Vec<PublicOfferingFundUse> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20260310002820");
    }
}
