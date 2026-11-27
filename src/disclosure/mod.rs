//! 공시정보 (disclosure information), OpenDART group `DS001`.
//!
//! | Endpoint | Method | 개발가이드 |
//! |---|---|---|
//! | `list.json` | [`Client::list`] | [공시검색](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS001&apiId=2019001) |
//! | `company.json` | [`Client::company`] | [기업개황](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS001&apiId=2019002) |
//! | `document.xml` | [`Client::document`] | [공시서류원본파일](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS001&apiId=2019003) |
//! | `corpCode.xml` | [`Client::corp_code`] | [고유번호](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS001&apiId=2019018) |
//!
//! [`Client::list`]: crate::Client::list
//! [`Client::company`]: crate::Client::company
//! [`Client::document`]: crate::Client::document
//! [`Client::corp_code`]: crate::Client::corp_code

mod company;
mod corp_code;
mod document;
mod list;

pub use company::Company;
pub use corp_code::CorpCodeEntry;
pub use document::DocumentFile;
pub(crate) use document::files as zip_files;
pub use list::{Disclosure, DisclosurePage, DisclosureType, ListRequest, SortKey, SortOrder};
