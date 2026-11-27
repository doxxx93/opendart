//! Test helper: checks endpoint names and request parameters against the
//! committed guide snapshot, `spec/guide-snapshot.json`.
//!
//! OpenDART ignores parameters it does not know instead of rejecting them,
//! so a misspelled name would silently return unfiltered data.

use std::sync::OnceLock;

use serde_json::Value;

use crate::client::{BASE_URL, Query};

fn snapshot() -> &'static Value {
    static SNAPSHOT: OnceLock<Value> = OnceLock::new();
    SNAPSHOT
        .get_or_init(|| serde_json::from_str(include_str!("../spec/guide-snapshot.json")).unwrap())
}

/// Panics unless the guide lists `endpoint`, and `query` uses only its
/// documented parameters and includes every required one.
pub(crate) fn assert_documented(endpoint: &str, query: &Query) {
    let url = format!("{BASE_URL}{endpoint}");
    let api = snapshot()["apis"]
        .as_array()
        .unwrap()
        .iter()
        .find(|api| {
            api["endpoints"]
                .as_array()
                .unwrap()
                .iter()
                .any(|e| e["url"] == url.as_str())
        })
        .unwrap_or_else(|| panic!("{endpoint} is not in the guide"));
    let params = api["request"].as_array().unwrap();
    for (key, _) in query {
        assert!(
            params.iter().any(|param| param["key"] == *key),
            "{endpoint}: `{key}` is not a documented parameter"
        );
    }
    for param in params {
        let key = param["key"].as_str().unwrap();
        if param["required"] == "Y" && key != "crtfc_key" {
            assert!(
                query.iter().any(|(name, _)| *name == key),
                "{endpoint}: the required parameter `{key}` is missing"
            );
        }
    }
}
