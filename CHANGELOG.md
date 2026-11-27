# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `Client` on reqwest (by default a 10 s connect and 30 s read timeout), with `rustls` (default), `rustls-no-provider` and `native-tls` features and `Client::from_reqwest` for a caller-built `reqwest::Client`.
- 공시정보 (`DS001`) endpoints in `opendart::disclosure`: `list.json` (`Client::list`), `company.json` (`Client::company`), `document.xml` (`Client::document`) and `corpCode.xml` (`Client::corp_code`).
- 정기보고서 주요정보 (`DS002`) endpoints in `opendart::periodic`: all 30, including the `...V2` pay endpoints added in 2026-05. They take a `ReportRequest` (`corp_code`, `bsns_year`, `ReportCode`) and return rows, or an empty list when the report has no such table.
- `Number` for amounts, counts and ratios: the text OpenDART sent, with `Number::parse` into any `FromStr` type after removing thousands separators and a trailing `%`.
- `Error` and `Status`, mapping OpenDART status codes; `Error::is_rate_limited` for status `020`. Every `Error` variant names the endpoint that failed (`Error::endpoint`).
