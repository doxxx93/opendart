//! 대량보유 상황보고 (`majorstock.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::{Client, Error, Number, decode};

const ENDPOINT: &str = "majorstock.json";

/// One report of a holding of 5% or more of a company's shares (주식등의
/// 대량보유상황보고), returned by [`Client::majorstock`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct MajorHolding {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 접수일자
    #[serde(deserialize_with = "decode::any_date")]
    pub rcept_dt: Date,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 보고구분, e.g. `일반`.
    #[serde(default)]
    pub report_tp: String,
    /// 대표보고자
    #[serde(default)]
    pub repror: String,
    /// 보유주식등의 수
    #[serde(default, deserialize_with = "decode::number")]
    pub stkqy: Option<Number>,
    /// 보유주식등의 증감
    #[serde(default, deserialize_with = "decode::number")]
    pub stkqy_irds: Option<Number>,
    /// 보유비율 (%)
    #[serde(default, deserialize_with = "decode::number")]
    pub stkrt: Option<Number>,
    /// 보유비율 증감 (%p)
    #[serde(default, deserialize_with = "decode::number")]
    pub stkrt_irds: Option<Number>,
    /// 주요체결 주식등의 수
    #[serde(default, deserialize_with = "decode::number")]
    pub ctr_stkqy: Option<Number>,
    /// 주요체결 보유비율 (%)
    #[serde(default, deserialize_with = "decode::number")]
    pub ctr_stkrt: Option<Number>,
    /// 보고사유: one line per reason, each starting with `- `.
    #[serde(default)]
    pub report_resn: String,
}

impl Client {
    /// 대량보유 상황보고 (`majorstock.json`): every report of a holding of
    /// 5% or more of the shares of `corp_code` (the 5% rule).
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn majorstock(&self, corp_code: &str) -> Result<Vec<MajorHolding>, Error> {
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
        let body = include_bytes!("../../tests/fixtures/ownership/majorstock.json");
        let rows: Vec<MajorHolding> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 4);
        // The guide says YYYYMMDD; the replies use YYYY-MM-DD.
        assert_eq!(rows[0].rcept_dt, date(2024, 12, 20));
        assert_eq!(rows[0].repror, "삼성물산");
        assert!(rows[0].stkqy_irds.as_ref().unwrap().parse::<i64>().unwrap() < 0);
    }
}
