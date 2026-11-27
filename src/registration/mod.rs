//! 증권신고서 주요정보 (securities registration statements), OpenDART group
//! `DS006`.
//!
//! Every endpoint takes a [`DateRangeRequest`](crate::DateRangeRequest) and
//! returns the statements filed in the range, grouped the way the reply
//! groups them (일반사항, 인수인정보, 자금의사용목적 and so on), one `Vec`
//! per group. No statements give empty groups.
//!
//! | Endpoint | Method | Statement | 개발가이드 |
//! |---|---|---|---|
//! | `estkRs.json` | [`Client::estk_rs`] | [`EquityRegistration`] | [지분증권](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS006&apiId=2020054) |
//! | `bdRs.json` | [`Client::bd_rs`] | [`DebtRegistration`] | [채무증권](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS006&apiId=2020055) |
//! | `stkdpRs.json` | [`Client::stkdp_rs`] | [`EquityRegistration`] | [증권예탁증권](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS006&apiId=2020056) |
//! | `mgRs.json` | [`Client::mg_rs`] | [`RestructuringRegistration`] | [합병](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS006&apiId=2020057) |
//! | `extrRs.json` | [`Client::extr_rs`] | [`RestructuringRegistration`] | [주식의포괄적교환·이전](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS006&apiId=2020058) |
//! | `dvRs.json` | [`Client::dv_rs`] | [`RestructuringRegistration`] | [분할](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS006&apiId=2020059) |
//!
//! [`Client::estk_rs`]: crate::Client::estk_rs
//! [`Client::bd_rs`]: crate::Client::bd_rs
//! [`Client::stkdp_rs`]: crate::Client::stkdp_rs
//! [`Client::mg_rs`]: crate::Client::mg_rs
//! [`Client::extr_rs`]: crate::Client::extr_rs
//! [`Client::dv_rs`]: crate::Client::dv_rs

mod endpoints;
mod groups;
mod rows;
mod types;

pub use rows::{
    BondGeneral, FundUse, IssuedSecurity, OfferedSecurity, OfferingGeneral, PartyCompany,
    PutBackRight, RestructuringGeneral, Seller, Underwriter,
};
pub use types::{DebtRegistration, EquityRegistration, RestructuringRegistration};
