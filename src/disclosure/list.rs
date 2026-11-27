//! 공시검색 (`list.json`).

use jiff::civil::Date;
use serde::Deserialize;

use crate::client::Query;
use crate::{Client, CorpClass, Error, Status, decode};

const ENDPOINT: &str = "list.json";

/// Disclosure type (`pblntf_ty`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DisclosureType {
    /// `A`: 정기공시 (periodic reports).
    Periodic,
    /// `B`: 주요사항보고 (major event reports).
    MajorEvent,
    /// `C`: 발행공시 (securities issuance).
    Issuance,
    /// `D`: 지분공시 (equity holdings).
    Equity,
    /// `E`: 기타공시 (other).
    Etc,
    /// `F`: 외부감사관련 (external audit).
    ExternalAudit,
    /// `G`: 펀드공시 (funds).
    Fund,
    /// `H`: 자산유동화 (asset-backed securities).
    AssetBacked,
    /// `I`: 거래소공시 (exchange disclosures).
    Exchange,
    /// `J`: 공정위공시 (Fair Trade Commission).
    FairTrade,
}

impl DisclosureType {
    /// The raw one-letter code, e.g. `"A"`.
    pub fn code(self) -> &'static str {
        match self {
            DisclosureType::Periodic => "A",
            DisclosureType::MajorEvent => "B",
            DisclosureType::Issuance => "C",
            DisclosureType::Equity => "D",
            DisclosureType::Etc => "E",
            DisclosureType::ExternalAudit => "F",
            DisclosureType::Fund => "G",
            DisclosureType::AssetBacked => "H",
            DisclosureType::Exchange => "I",
            DisclosureType::FairTrade => "J",
        }
    }
}

/// Sort key (`sort`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SortKey {
    /// `date`: receipt date (the OpenDART default).
    Date,
    /// `crp`: company name.
    Company,
    /// `rpt`: report name.
    Report,
}

impl SortKey {
    /// The raw value, e.g. `"crp"`.
    pub fn code(self) -> &'static str {
        match self {
            SortKey::Date => "date",
            SortKey::Company => "crp",
            SortKey::Report => "rpt",
        }
    }
}

/// Sort order (`sort_mth`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SortOrder {
    /// `asc`.
    Asc,
    /// `desc` (the OpenDART default).
    Desc,
}

impl SortOrder {
    /// The raw value, e.g. `"asc"`.
    pub fn code(self) -> &'static str {
        match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        }
    }
}

/// Search conditions for [`Client::list`].
///
/// Setters are named after the OpenDART request parameters. Every
/// parameter is optional; unset ones use the OpenDART defaults.
///
/// ```
/// use opendart::disclosure::{DisclosureType, ListRequest};
/// use opendart::jiff::civil::date;
///
/// let request = ListRequest::new()
///     .bgn_de(date(2026, 11, 1))
///     .end_de(date(2026, 11, 27))
///     .pblntf_ty(DisclosureType::Exchange)
///     .page_count(100);
/// ```
#[derive(Debug, Clone, Default)]
pub struct ListRequest {
    corp_code: Option<String>,
    bgn_de: Option<Date>,
    end_de: Option<Date>,
    last_reprt_at: Option<bool>,
    pblntf_ty: Option<DisclosureType>,
    pblntf_detail_ty: Option<String>,
    corp_cls: Option<CorpClass>,
    sort: Option<SortKey>,
    sort_mth: Option<SortOrder>,
    page_no: Option<u32>,
    page_count: Option<u32>,
}

impl ListRequest {
    /// A request with no conditions: today's disclosures, newest first.
    pub fn new() -> Self {
        Self::default()
    }

    /// Only this company (`corp_code`, 8 digits; see
    /// [`Client::corp_code`]).
    pub fn corp_code(mut self, corp_code: impl Into<String>) -> Self {
        self.corp_code = Some(corp_code.into());
        self
    }

    /// First receipt date to include (`bgn_de`). Defaults to `end_de`.
    ///
    /// Without `corp_code`, OpenDART limits the range to three months.
    pub fn bgn_de(mut self, date: Date) -> Self {
        self.bgn_de = Some(date);
        self
    }

    /// Last receipt date to include (`end_de`). Defaults to today.
    pub fn end_de(mut self, date: Date) -> Self {
        self.end_de = Some(date);
        self
    }

    /// Whether to return only the final version of each report
    /// (`last_reprt_at`). Defaults to `false`, which includes amendments.
    pub fn last_reprt_at(mut self, final_only: bool) -> Self {
        self.last_reprt_at = Some(final_only);
        self
    }

    /// Disclosure type (`pblntf_ty`).
    pub fn pblntf_ty(mut self, kind: DisclosureType) -> Self {
        self.pblntf_ty = Some(kind);
        self
    }

    /// Detailed disclosure type (`pblntf_detail_ty`), such as `"A001"` for
    /// 사업보고서. The codes are listed in the 공시검색 guide.
    pub fn pblntf_detail_ty(mut self, code: impl Into<String>) -> Self {
        self.pblntf_detail_ty = Some(code.into());
        self
    }

    /// Market (`corp_cls`). Only one market per request.
    ///
    /// OpenDART ignores codes it does not know instead of rejecting them.
    pub fn corp_cls(mut self, class: CorpClass) -> Self {
        self.corp_cls = Some(class);
        self
    }

    /// Sort key (`sort`).
    pub fn sort(mut self, key: SortKey) -> Self {
        self.sort = Some(key);
        self
    }

    /// Sort order (`sort_mth`).
    pub fn sort_mth(mut self, order: SortOrder) -> Self {
        self.sort_mth = Some(order);
        self
    }

    /// Page number, starting at 1 (`page_no`).
    pub fn page_no(mut self, page_no: u32) -> Self {
        self.page_no = Some(page_no);
        self
    }

    /// Results per page, 1 to 100 (`page_count`). Defaults to 10.
    pub fn page_count(mut self, page_count: u32) -> Self {
        self.page_count = Some(page_count);
        self
    }

    fn query(&self) -> Query {
        let mut query = Query::new();
        let mut push = |name, value: Option<String>| {
            if let Some(value) = value {
                query.push((name, value));
            }
        };
        push("corp_code", self.corp_code.clone());
        push("bgn_de", self.bgn_de.map(decode::format_date));
        push("end_de", self.end_de.map(decode::format_date));
        push(
            "last_reprt_at",
            self.last_reprt_at
                .map(|yes| if yes { "Y" } else { "N" }.to_owned()),
        );
        push("pblntf_ty", self.pblntf_ty.map(|t| t.code().to_owned()));
        push("pblntf_detail_ty", self.pblntf_detail_ty.clone());
        push(
            "corp_cls",
            self.corp_cls.as_ref().map(|c| c.code().to_owned()),
        );
        push("sort", self.sort.map(|s| s.code().to_owned()));
        push("sort_mth", self.sort_mth.map(|s| s.code().to_owned()));
        push("page_no", self.page_no.map(|n| n.to_string()));
        push("page_count", self.page_count.map(|n| n.to_string()));
        query
    }

    /// The page OpenDART would return when nothing matches.
    fn empty_page(&self) -> DisclosurePage {
        DisclosurePage {
            page_no: self.page_no.unwrap_or(1),
            page_count: self.page_count.unwrap_or(10),
            total_count: 0,
            total_page: 0,
            list: Vec::new(),
        }
    }
}

/// One page of [`Client::list`] results.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct DisclosurePage {
    /// Page number (`page_no`).
    pub page_no: u32,
    /// Results per page (`page_count`).
    pub page_count: u32,
    /// Total number of matching disclosures (`total_count`).
    pub total_count: u32,
    /// Total number of pages (`total_page`).
    pub total_page: u32,
    /// The disclosures on this page (`list`).
    pub list: Vec<Disclosure>,
}

/// One disclosure in [`DisclosurePage::list`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct Disclosure {
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 종목명 for listed companies, 법인명 otherwise.
    pub corp_name: String,
    /// 종목코드: the 6-character stock code, `None` for unlisted companies.
    #[serde(default, deserialize_with = "decode::opt_string")]
    pub stock_code: Option<String>,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 보고서명: report name, with prefixes such as `[기재정정]` for
    /// amendments. Trailing padding is removed.
    #[serde(deserialize_with = "decode::trimmed")]
    pub report_nm: String,
    /// 접수번호: the 14-digit receipt number that identifies the filing.
    pub rcept_no: String,
    /// 공시 제출인명: the filer.
    pub flr_nm: String,
    /// 접수일자: the receipt date. OpenDART gives no time of day.
    #[serde(deserialize_with = "decode::date")]
    pub rcept_dt: Date,
    /// 비고: zero or more one-character flags, e.g. `"유정"`. `유`, `코`,
    /// `채`, `넥` name the KRX market division, `공` the Fair Trade
    /// Commission, `연` a consolidated report, `정` a later amendment and
    /// `철` a withdrawal.
    pub rm: String,
}

impl Disclosure {
    /// The DART web viewer page for this filing.
    pub fn viewer_url(&self) -> String {
        format!(
            "https://dart.fss.or.kr/dsaf001/main.do?rcpNo={}",
            self.rcept_no
        )
    }
}

impl Client {
    /// 공시검색: searches disclosures (`list.json`).
    ///
    /// A search with no matches returns an empty page instead of
    /// [`Status::NoData`].
    pub async fn list(&self, request: &ListRequest) -> Result<DisclosurePage, Error> {
        let body = self.get(ENDPOINT, &request.query()).await?;
        parse(request, &body)
    }
}

fn parse(request: &ListRequest, body: &[u8]) -> Result<DisclosurePage, Error> {
    match decode::json(ENDPOINT, body) {
        Err(Error::Api {
            status: Status::NoData,
            ..
        }) => Ok(request.empty_page()),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn query_uses_opendart_names_and_formats() {
        let request = ListRequest::new()
            .corp_code("00126380")
            .bgn_de(date(2026, 1, 2))
            .end_de(date(2026, 11, 27))
            .last_reprt_at(true)
            .pblntf_ty(DisclosureType::Periodic)
            .pblntf_detail_ty("A003")
            .corp_cls(CorpClass::Kospi)
            .sort(SortKey::Company)
            .sort_mth(SortOrder::Asc)
            .page_no(2)
            .page_count(100);
        crate::guide_check::assert_documented(ENDPOINT, &request.query());
        crate::guide_check::assert_documented(ENDPOINT, &ListRequest::new().query());
        let query = request.query();
        let query: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        assert_eq!(
            query,
            [
                ("corp_code", "00126380"),
                ("bgn_de", "20260102"),
                ("end_de", "20261127"),
                ("last_reprt_at", "Y"),
                ("pblntf_ty", "A"),
                ("pblntf_detail_ty", "A003"),
                ("corp_cls", "Y"),
                ("sort", "crp"),
                ("sort_mth", "asc"),
                ("page_no", "2"),
                ("page_count", "100"),
            ]
        );
        assert!(ListRequest::new().query().is_empty());
    }

    #[test]
    fn parses_an_exchange_page() {
        let body = include_bytes!("../../tests/fixtures/list/exchange_page.json");
        let page = parse(&ListRequest::new(), body).unwrap();
        assert_eq!(
            (
                page.page_no,
                page.page_count,
                page.total_count,
                page.total_page
            ),
            (1, 5, 53, 11)
        );
        assert_eq!(page.list.len(), 5);

        let first = &page.list[0];
        assert_eq!(first.rcept_no, "20261126800950");
        assert_eq!(first.corp_cls, CorpClass::Kospi);
        assert_eq!(first.report_nm, "[기재정정]타인에대한채무보증결정");
        assert_eq!(first.rcept_dt, date(2026, 11, 26));
        assert_eq!(first.rm, "유");
        assert_eq!(page.list[1].rm, "유정");
        assert_eq!(
            first.viewer_url(),
            "https://dart.fss.or.kr/dsaf001/main.do?rcpNo=20261126800950"
        );
    }

    #[test]
    fn parses_a_company_page() {
        let body = include_bytes!("../../tests/fixtures/list/corp_periodic.json");
        let page = parse(&ListRequest::new(), body).unwrap();
        let first = &page.list[0];
        assert_eq!(first.corp_code, "00126380");
        assert_eq!(first.stock_code.as_deref(), Some("005930"));
        assert_eq!(first.report_nm, "분기보고서 (2026.09)");
        assert_eq!(first.rm, "");
    }

    #[test]
    fn no_data_is_an_empty_page() {
        let body = include_bytes!("../../tests/fixtures/list/no_data.json");
        let request = ListRequest::new().page_no(3).page_count(20);
        let page = parse(&request, body).unwrap();
        assert_eq!(page.list, []);
        assert_eq!(
            (page.page_no, page.page_count, page.total_count),
            (3, 20, 0)
        );
    }

    #[test]
    fn other_statuses_stay_errors() {
        let body = include_bytes!("../../tests/fixtures/error/unregistered_key.json");
        let err = parse(&ListRequest::new(), body).unwrap_err();
        assert_eq!(err.status(), Some(&Status::UnregisteredKey));
    }
}
