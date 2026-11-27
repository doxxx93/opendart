# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `Client` on reqwest (by default a 10 s connect and 30 s read timeout), with `rustls` (default), `rustls-no-provider` and `native-tls` features and `Client::from_reqwest` for a caller-built `reqwest::Client`.
- 공시정보 (`DS001`) endpoints in `opendart::disclosure`: `list.json` (`Client::list`), `company.json` (`Client::company`), `document.xml` (`Client::document`) and `corpCode.xml` (`Client::corp_code`).
- 정기보고서 주요정보 (`DS002`) endpoints in `opendart::periodic`: all 30, including the `...V2` pay endpoints added in 2026-05. They take a `ReportRequest` (`corp_code`, `bsns_year`, `ReportCode`) and return rows, or an empty list when the report has no such table; the two `...V2` endpoints return one `Option<PayReport>`.
- 정기보고서 재무정보 (`DS003`) endpoints in `opendart::financial`: all 7. Multi-company endpoints take a `MultiReportRequest`; `fnlttXbrl.xml` returns the XBRL files as `DocumentFile`s.
- 지분공시 종합정보 (`DS004`) endpoints in `opendart::ownership`: `majorstock.json` (`Client::majorstock`) and `elestock.json` (`Client::elestock`).
- 주요사항보고서 주요정보 (`DS005`) endpoints in `opendart::major_event`: all 36. They take a `DateRangeRequest` (`corp_code`, `bgn_de`, `end_de`) and return one row per report.
- 증권신고서 주요정보 (`DS006`) endpoints in `opendart::registration`: all 6. They take a `DateRangeRequest` and return one value per statement (`EquityRegistration`, `DebtRegistration`, `RestructuringRegistration`) with a `Vec` for each group of the reply.
- `Number` for amounts, counts and ratios: the text OpenDART sent, with `Number::parse` into any `FromStr` type after removing thousands separators and a trailing `%`.
- `Error` and `Status`, mapping OpenDART status codes; `Error::is_rate_limited` for status `020`. Every `Error` variant names the endpoint that failed (`Error::endpoint`).
