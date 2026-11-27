//! 영업정지 (`bsnSp.json`).

use serde::Deserialize;

use crate::{Client, CorpClass, DateRangeRequest, Error, Number, decode};

const ENDPOINT: &str = "bsnSp.json";

/// One 영업정지 report: suspensions of business, returned by [`Client::bsn_sp`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct BusinessSuspension {
    /// 접수번호: the 14-digit receipt number of the report.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 영업정지 분야
    #[serde(default)]
    pub bsnsp_rm: String,
    /// 영업정지 내역(영업정지금액)
    #[serde(default, deserialize_with = "decode::number")]
    pub bsnsp_amt: Option<Number>,
    /// 영업정지 내역(최근매출총액)
    #[serde(default, deserialize_with = "decode::number")]
    pub rsl: Option<Number>,
    /// 영업정지 내역(매출액 대비)
    #[serde(default, deserialize_with = "decode::number")]
    pub sl_vs: Option<Number>,
    /// 영업정지 내역(대규모법인여부)
    #[serde(default)]
    pub ls_atn: String,
    /// 영업정지 내역(거래소 의무공시 해당 여부)
    #[serde(default)]
    pub krx_stt_atn: String,
    /// 영업정지 내용
    #[serde(default)]
    pub bsnsp_cn: String,
    /// 영업정지사유
    #[serde(default)]
    pub bsnsp_rs: String,
    /// 향후대책
    #[serde(default)]
    pub ft_ctp: String,
    /// 영업정지영향
    #[serde(default)]
    pub bsnsp_af: String,
    /// 영업정지일자
    #[serde(default)]
    pub bsnspd: String,
    /// 이사회결의일(결정일)
    #[serde(default)]
    pub bddd: String,
    /// 독립(사외)이사 참석여부(참석)
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_t: Option<Number>,
    /// 독립(사외)이사 참석여부(불참)
    #[serde(default, deserialize_with = "decode::number")]
    pub od_a_at_b: Option<Number>,
    /// 감사(감사위원) 참석여부
    #[serde(default)]
    pub adt_a_atn: String,
}

impl Client {
    /// 영업정지 (`bsnSp.json`): suspensions of business filed in the date range.
    ///
    /// Returns an empty list when there are none
    /// ([`Status::NoData`](crate::Status::NoData)).
    pub async fn bsn_sp(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<BusinessSuspension>, Error> {
        self.date_range_rows(ENDPOINT, request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_matches_the_guide() {
        crate::guide_check::assert_documented(ENDPOINT, &crate::date_range::example().query());
    }

    #[test]
    fn decodes_fixture() {
        let body = include_bytes!("../../tests/fixtures/major_event/bsnSp.json");
        let rows: Vec<BusinessSuspension> = decode::rows(ENDPOINT, body).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].rcept_no, "20190731000609");
        assert_eq!(
            rows[0].bsnsp_amt.as_ref().map(Number::as_str),
            Some("12,704,835,489")
        );
    }
}
