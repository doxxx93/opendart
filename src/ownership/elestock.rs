//! 임원ㆍ주요주주 소유보고 (`elestock.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, Error, Number, decode};

const ENDPOINT: &str = "elestock.json";

/// One report of shares owned by an executive or major shareholder
/// (임원ㆍ주요주주 특정증권등 소유상황보고), returned by [`Client::elestock`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct InsiderHolding {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 접수일자
    #[serde(deserialize_with = "decode::any_date")]
    pub rcept_dt: Date,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 보고자: the person or company that filed.
    #[serde(default)]
    pub repror: String,
    /// 발행 회사 관계 임원(등기여부), e.g. `등기임원`, `비등기임원`.
    #[serde(default)]
    pub isu_exctv_rgist_at: String,
    /// 발행 회사 관계 임원 직위, e.g. `대표이사`, `상무`.
    #[serde(default)]
    pub isu_exctv_ofcps: String,
    /// 발행 회사 관계 주요 주주, e.g. `10%이상주주`; `-` for none.
    #[serde(default)]
    pub isu_main_shrholdr: String,
    /// 특정 증권 등 소유 수
    #[serde(default, deserialize_with = "decode::number")]
    pub sp_stock_lmp_cnt: Option<Number>,
    /// 특정 증권 등 소유 증감 수
    #[serde(default, deserialize_with = "decode::number")]
    pub sp_stock_lmp_irds_cnt: Option<Number>,
    /// 특정 증권 등 소유 비율 (%)
    #[serde(default, deserialize_with = "decode::number")]
    pub sp_stock_lmp_rate: Option<Number>,
    /// 특정 증권 등 소유 증감 비율 (%p)
    #[serde(default, deserialize_with = "decode::number")]
    pub sp_stock_lmp_irds_rate: Option<Number>,
}

impl Client {
    /// 임원ㆍ주요주주 소유보고 (`elestock.json`): every report of shares
    /// owned by executives and major shareholders of `corp_code`.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn elestock(&self, corp_code: &str) -> Result<Vec<InsiderHolding>, Error> {
        self.rows(ENDPOINT, &super::query(corp_code)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn request_matches_the_guide() {
        crate::guide_check::assert_documented(ENDPOINT, &super::super::query("00126380"));
    }

    #[test]
    fn decodes_fixture() {
        let body = include_bytes!("../../tests/fixtures/ownership/elestock.json");
        let rows: Vec<InsiderHolding> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].rcept_dt, date(2024, 12, 2));
        assert!(
            rows[0]
                .sp_stock_lmp_cnt
                .as_ref()
                .unwrap()
                .parse::<i64>()
                .is_ok()
        );
    }
}
