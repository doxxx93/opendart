//! 정기보고서 재무정보 (financial statements in periodic reports), OpenDART
//! group `DS003`.
//!
//! | Endpoint | Method | Row | 개발가이드 |
//! |---|---|---|---|
//! | `fnlttSinglAcnt.json` | [`Client::fnltt_singl_acnt`] | [`Account`] | [단일회사 주요계정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS003&apiId=2019016) |
//! | `fnlttMultiAcnt.json` | [`Client::fnltt_multi_acnt`] | [`Account`] | [다중회사 주요계정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS003&apiId=2019017) |
//! | `fnlttXbrl.xml` | [`Client::fnltt_xbrl`] | [`DocumentFile`](crate::disclosure::DocumentFile) | [재무제표 원본파일(XBRL)](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS003&apiId=2019019) |
//! | `fnlttSinglAcntAll.json` | [`Client::fnltt_singl_acnt_all`] | [`StatementLine`] | [단일회사 전체 재무제표](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS003&apiId=2019020) |
//! | `xbrlTaxonomy.json` | [`Client::xbrl_taxonomy`] | [`TaxonomyAccount`] | [XBRL택사노미재무제표양식](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS003&apiId=2020001) |
//! | `fnlttSinglIndx.json` | [`Client::fnltt_singl_indx`] | [`Indicator`] | [단일회사 주요 재무지표](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS003&apiId=2022001) |
//! | `fnlttCmpnyIndx.json` | [`Client::fnltt_cmpny_indx`] | [`Indicator`] | [다중회사 주요 재무지표](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS003&apiId=2022002) |
//!
//! [`Client::fnltt_singl_acnt`]: crate::Client::fnltt_singl_acnt
//! [`Client::fnltt_multi_acnt`]: crate::Client::fnltt_multi_acnt
//! [`Client::fnltt_xbrl`]: crate::Client::fnltt_xbrl
//! [`Client::fnltt_singl_acnt_all`]: crate::Client::fnltt_singl_acnt_all
//! [`Client::xbrl_taxonomy`]: crate::Client::xbrl_taxonomy
//! [`Client::fnltt_singl_indx`]: crate::Client::fnltt_singl_indx
//! [`Client::fnltt_cmpny_indx`]: crate::Client::fnltt_cmpny_indx

mod fnltt_cmpny_indx;
mod fnltt_multi_acnt;
mod fnltt_singl_acnt;
mod fnltt_singl_acnt_all;
mod fnltt_singl_indx;
mod fnltt_xbrl;
mod xbrl_taxonomy;

pub use fnltt_singl_acnt::Account;
pub use fnltt_singl_acnt_all::{Consolidation, StatementLine};
pub use fnltt_singl_indx::{Indicator, IndicatorClass};
pub use xbrl_taxonomy::TaxonomyAccount;
