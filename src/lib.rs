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
//! Modules follow the API groups of the [개발가이드], so that
//! [`disclosure`] is 공시정보 (`DS001`). Methods are the endpoint names in
//! snake case, and request setters and response fields keep the OpenDART
//! parameter names (`rcept_no`, `bgn_de`, `induty_code`), so the official
//! guide can be read side by side with this crate.
//!
//! # Values
//!
//! Dates are [`jiff::civil::Date`]. Blank stock codes and similar optional
//! values become `None`; other text passes through unchanged. OpenDART
//! reports errors with a `status` code inside the body, which surfaces as
//! [`Error::Api`].
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
mod decode;
pub mod disclosure;
mod error;

pub use client::Client;
pub use corp_class::CorpClass;
pub use error::{Error, Status};
/// Re-export of the `jiff` version used for dates.
pub use jiff;
/// Re-export of the `reqwest` version [`Client::from_reqwest`] takes.
pub use reqwest;
