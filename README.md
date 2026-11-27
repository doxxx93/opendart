# opendart

Typed Rust client for [OpenDART](https://opendart.fss.or.kr), the electronic
disclosure (DART) API of Korea's Financial Supervisory Service.

OpenDART(금융감독원 전자공시) API를 위한 Rust 클라이언트입니다.

## Status

Early development, not published yet. 0.1.0 ships once all 85 endpoints of
the guide are in.

## Scope

공시정보 (`DS001`), in `opendart::disclosure`:

- `list.json`: disclosure search
- `company.json`: company overview, including the KSIC industry code
- `document.xml`: the original documents of a filing (a ZIP)
- `corpCode.xml`: the corp code to stock code mapping (a ZIP holding one XML file)

정기보고서 주요정보 (`DS002`), in `opendart::periodic`: all 30 endpoints
(dividends, shareholders, executives and pay, employees, debt balances,
auditors, use of funds and others), each taking a `ReportRequest` for one
company's annual, half-year or quarterly report. Amounts and ratios are
`opendart::Number`, the text OpenDART sent, which you parse into `i64`,
`f64` or a decimal type.

정기보고서 재무정보 (`DS003`), in `opendart::financial`: key accounts and
financial indicators for one or up to 100 companies, every line of the full
statements, the XBRL files of a report and the XBRL taxonomy forms.

지분공시 종합정보 (`DS004`), in `opendart::ownership`: 5% holding reports
(`majorstock.json`) and executive and major shareholder holdings
(`elestock.json`).

The other groups (major event reports, securities registration) come later.

## Usage

```rust
use opendart::Client;
use opendart::disclosure::ListRequest;

#[tokio::main]
async fn main() -> Result<(), opendart::Error> {
    let dart = Client::new(std::env::var("OPENDART_API_KEY").unwrap());
    let page = dart.list(&ListRequest::new().page_count(100)).await?;
    for filing in &page.list {
        println!("{} {} {}", filing.rcept_dt, filing.corp_name, filing.report_nm);
    }
    Ok(())
}
```

Request setters and response fields keep the OpenDART parameter names, so
the [official guide](https://opendart.fss.or.kr/guide/main.do) applies as is.
The crate does not throttle requests. OpenDART allows a personal key 20,000
requests a day and blocks an IP address for an hour after more than 1,000
requests in a minute (see the crate docs for details).

## API key

OpenDART requires a free API key from <https://opendart.fss.or.kr>. Never
commit it. Tests that call the live API read `OPENDART_API_KEY` from the
environment and are marked `#[ignore]`, so `cargo test` runs offline.

## Minimum supported Rust version

Rust 1.88 (edition 2024). Raising the MSRV is treated as a minor version
change.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
