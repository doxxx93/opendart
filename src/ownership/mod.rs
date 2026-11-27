//! 지분공시 종합정보 (share ownership reports), OpenDART group `DS004`.
//!
//! | Endpoint | Method | Row | 개발가이드 |
//! |---|---|---|---|
//! | `majorstock.json` | [`Client::majorstock`] | [`MajorHolding`] | [대량보유 상황보고](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS004&apiId=2019021) |
//! | `elestock.json` | [`Client::elestock`] | [`InsiderHolding`] | [임원ㆍ주요주주 소유보고](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS004&apiId=2019022) |
//!
//! Both take only a `corp_code` and return every report filed about that
//! company, which for a large company is thousands of rows.
//!
//! [`Client::majorstock`]: crate::Client::majorstock
//! [`Client::elestock`]: crate::Client::elestock

mod elestock;
mod majorstock;

pub use elestock::InsiderHolding;
pub use majorstock::MajorHolding;

use crate::client::Query;

fn query(corp_code: &str) -> Query {
    vec![("corp_code", corp_code.to_owned())]
}
