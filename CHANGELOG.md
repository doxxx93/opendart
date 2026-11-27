# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `Client` on reqwest (by default a 10 s connect and 30 s read timeout), with `rustls` (default), `rustls-no-provider` and `native-tls` features and `Client::from_reqwest` for a caller-built `reqwest::Client`.
- 공시정보 (`DS001`) endpoints in `opendart::disclosure`: `list.json` (`Client::list`), `company.json` (`Client::company`), `document.xml` (`Client::document`) and `corpCode.xml` (`Client::corp_code`).
- `Error` and `Status`, mapping OpenDART status codes; `Error::is_rate_limited` for status `020`.
