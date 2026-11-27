//! Tests against the live OpenDART API. They are ignored by default; run
//! them with a key:
//!
//! ```sh
//! OPENDART_API_KEY=... cargo test --test live -- --ignored
//! ```

use opendart::disclosure::ListRequest;
use opendart::financial::Consolidation;
use opendart::jiff::civil::date;
use opendart::{Client, DateRangeRequest, ReportCode, ReportRequest, Status};

fn client() -> Client {
    Client::new(std::env::var("OPENDART_API_KEY").expect("OPENDART_API_KEY is not set"))
}

#[tokio::test]
#[ignore = "calls the live OpenDART API"]
async fn list_one_day() {
    let request = ListRequest::new()
        .bgn_de(date(2026, 11, 26))
        .end_de(date(2026, 11, 26))
        .page_count(5);
    let page = client().list(&request).await.unwrap();
    assert_eq!(page.page_count, 5);
    assert!(page.total_count > 0);
    assert!(page.list.iter().all(|d| d.rcept_dt == date(2026, 11, 26)));
}

#[tokio::test]
#[ignore = "calls the live OpenDART API"]
async fn company_overview() {
    let company = client().company("00126380").await.unwrap();
    assert_eq!(company.stock_code.as_deref(), Some("005930"));
}

#[tokio::test]
#[ignore = "calls the live OpenDART API"]
async fn corp_code_listing() {
    let entries = client().corp_code().await.unwrap();
    assert!(entries.len() > 100_000, "{} entries", entries.len());
    assert!(
        entries
            .iter()
            .any(|e| e.corp_code == "00126380" && e.stock_code.as_deref() == Some("005930"))
    );
}

#[tokio::test]
#[ignore = "calls the live OpenDART API"]
async fn document_files() {
    let files = client().document("20261126800680").await.unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].name, "20261126800680.xml");
}

#[tokio::test]
#[ignore = "calls the live OpenDART API"]
async fn dividends() {
    let request = ReportRequest::new("00126380", 2025, ReportCode::Annual);
    let rows = client().alot_matter(&request).await.unwrap();
    let par_value = rows
        .iter()
        .find(|row| row.se == "주당액면가액(원)")
        .unwrap();
    assert_eq!(par_value.thstrm.as_ref().unwrap().parse::<i64>(), Ok(100));
}

#[tokio::test]
#[ignore = "calls the live OpenDART API"]
async fn individual_pay_v2() {
    let request = ReportRequest::new("00126380", 2026, ReportCode::HalfYear);
    let report = client().indvdl_by_pay_v2(&request).await.unwrap().unwrap();
    assert_eq!(report.corp_code, "00126380");
    assert!(!report.group.is_empty());
}

#[tokio::test]
#[ignore = "calls the live OpenDART API"]
async fn full_statements() {
    let request = ReportRequest::new("00126380", 2025, ReportCode::Annual);
    let lines = client()
        .fnltt_singl_acnt_all(&request, Consolidation::Consolidated)
        .await
        .unwrap();
    let assets = lines
        .iter()
        .find(|line| line.account_id == "ifrs-full_Assets")
        .unwrap();
    assert!(
        assets
            .thstrm_amount
            .as_ref()
            .unwrap()
            .parse::<i64>()
            .unwrap()
            > 0
    );
}

#[tokio::test]
#[ignore = "calls the live OpenDART API"]
async fn major_holdings() {
    let rows = client().majorstock("00126380").await.unwrap();
    assert!(rows.iter().any(|row| row.repror == "삼성물산"));
}

#[tokio::test]
#[ignore = "calls the live OpenDART API"]
async fn treasury_stock_decisions() {
    // The example in the guide: Hyundai Motor in 2019.
    let request = DateRangeRequest::new("00164742", date(2019, 1, 1), date(2019, 12, 31));
    let rows = client().tsstk_aq_decsn(&request).await.unwrap();
    assert!(!rows.is_empty());
    assert!(rows.iter().all(|row| row.corp_code == "00164742"));
}

#[tokio::test]
#[ignore = "calls the live OpenDART API"]
async fn merger_registration() {
    // The example in the guide.
    let request = DateRangeRequest::new("00109718", date(2019, 1, 1), date(2019, 12, 31));
    let statements = client().mg_rs(&request).await.unwrap();
    assert!(!statements.is_empty());
    assert!(
        statements
            .iter()
            .all(|s| !s.general.is_empty() && !s.companies.is_empty())
    );
}

#[tokio::test]
#[ignore = "calls the live OpenDART API"]
async fn unregistered_key() {
    let client = Client::new("0".repeat(40));
    let err = client.company("00126380").await.unwrap_err();
    assert_eq!(err.status(), Some(&Status::UnregisteredKey));
}
