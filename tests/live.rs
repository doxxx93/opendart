//! Tests against the live OpenDART API. They are ignored by default; run
//! them with a key:
//!
//! ```sh
//! OPENDART_API_KEY=... cargo test --test live -- --ignored
//! ```

use opendart::disclosure::ListRequest;
use opendart::jiff::civil::date;
use opendart::{Client, Status};

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
async fn unregistered_key() {
    let client = Client::new("0".repeat(40));
    let err = client.company("00126380").await.unwrap_err();
    assert_eq!(err.status(), Some(&Status::UnregisteredKey));
}
