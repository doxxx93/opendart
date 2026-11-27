//! Typed async client for [OpenDART], the electronic disclosure (DART) API
//! run by Korea's Financial Supervisory Service.
//!
//! ```no_run
//! use opendart::Client;
//! use opendart::disclosure::ListRequest;
//!
//! # async fn run() -> Result<(), opendart::Error> {
//! let dart = Client::new(std::env::var("OPENDART_API_KEY").unwrap());
//! let page = dart.list(&ListRequest::new().page_count(100)).await?;
//! for filing in &page.list {
//!     println!("{} {} {}", filing.rcept_dt, filing.corp_name, filing.report_nm);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Naming
//!
//! Modules follow the API groups of the [개발가이드]: [`disclosure`] is
//! 공시정보 (`DS001`), [`periodic`] is 정기보고서 주요정보 (`DS002`),
//! [`financial`] is 정기보고서 재무정보 (`DS003`), [`ownership`] is
//! 지분공시 종합정보 (`DS004`), [`major_event`] is 주요사항보고서 주요정보
//! (`DS005`) and [`registration`] is 증권신고서 주요정보 (`DS006`). Methods are the endpoint names in
//! snake case, and request setters and response fields keep the OpenDART
//! parameter names (`rcept_no`, `bgn_de`, `induty_code`), so the official
//! guide can be read side by side with this crate.
//!
//! # Values
//!
//! - Dates that OpenDART itself formats (`rcept_dt`, `stlm_dt`, `est_dt` and
//!   the like) are [`jiff::civil::Date`]. Dates that companies type into
//!   their reports (`bddd`, `sbd`, `isu_de`, often `2019년 02월 20일`,
//!   `1997.10.02` or a range) are text.
//! - Amounts, counts and ratios are [`Number`]s holding the text OpenDART
//!   sent, which the caller parses into the type it wants (`i64`, `f64`, a
//!   decimal type).
//! - Codes in replies (`reprt_code`, `fs_div`, `sj_div`, `idx_cl_code`) are
//!   strings as sent; the request parameters that take them are enums such
//!   as [`ReportCode`].
//! - Blank stock codes and similar optional values become `None`; other
//!   text passes through unchanged.
//!
//! Response types implement [`serde::Deserialize`] for OpenDART's wire
//! format, so serde is a public dependency. OpenDART reports errors with a
//! `status` code inside the body, which surfaces as [`Error::Api`].
//!
//! # Request limits
//!
//! This crate does not throttle requests. According to the OpenDART FAQ:
//!
//! - A personal key may make 20,000 requests a day across all endpoints.
//!   A business key has no daily limit on `list.json` and `company.json`
//!   and 20,000 a day across the rest. Past the limit, requests fail with
//!   [`Status::RateLimited`] (see [`Error::is_rate_limited`]).
//! - More than 1,000 requests a minute gets the client's IP address
//!   blocked from OpenDART for an hour, regardless of the daily limit.
//! - Scheduled maintenance suspends some endpoints for hours or days at a
//!   time (for example `document.xml` and `corpCode.xml`); the notices are
//!   on the OpenDART 공지사항 board.
//!
//! # Features
//!
//! - `rustls` (default): TLS through rustls with the aws-lc-rs crypto provider.
//! - `rustls-no-provider`: rustls without a crypto provider, for callers who
//!   install their own (such as ring) with `rustls::crypto::CryptoProvider`.
//! - `native-tls`: TLS through the platform library.
//!
//! With none of them, pass a [`reqwest::Client`] that has TLS to
//! [`Client::from_reqwest`].
//!
//! [OpenDART]: https://opendart.fss.or.kr
//! [개발가이드]: https://opendart.fss.or.kr/guide/main.do

mod client;
mod corp_class;
mod date_range;
mod decode;
pub mod disclosure;
mod error;
pub mod financial;
#[cfg(test)]
mod guide_check;
pub mod major_event;
mod number;
pub mod ownership;
pub mod periodic;
pub mod registration;
mod report;

pub use client::Client;
pub use corp_class::CorpClass;
pub use date_range::DateRangeRequest;
pub use error::{Error, Status};
/// Re-export of the `jiff` version used for dates.
pub use jiff;
pub use number::Number;
pub use report::{MultiReportRequest, ReportCode, ReportRequest};
/// Re-export of the `reqwest` version [`Client::from_reqwest`] takes.
pub use reqwest;
