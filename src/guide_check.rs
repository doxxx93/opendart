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

/// Every endpoint file name (`"list.json"`) assigned to a `&str` constant in
/// `src/`.
fn implemented() -> std::collections::HashSet<String> {
    fn walk(dir: &std::path::Path, names: &mut std::collections::HashSet<String>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, names);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                for line in std::fs::read_to_string(&path).unwrap().lines() {
                    let Some((_, value)) = line.split_once(": &str = \"") else {
                        continue;
                    };
                    if let Some(name) = value.strip_suffix("\";")
                        && (name.ends_with(".json") || name.ends_with(".xml"))
                    {
                        names.insert(name.to_owned());
                    }
                }
            }
        }
    }
    let mut names = std::collections::HashSet::new();
    walk(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut names,
    );
    names
}

/// The guide watch updates the snapshot when OpenDART adds an endpoint;
/// this then fails until the endpoint is implemented.
#[test]
fn every_guide_endpoint_is_implemented() {
    let names = implemented();
    let mut missing = Vec::new();
    for api in snapshot()["apis"].as_array().unwrap() {
        let files: Vec<&str> = api["endpoints"]
            .as_array()
            .unwrap()
            .iter()
            .map(|endpoint| {
                endpoint["url"]
                    .as_str()
                    .unwrap()
                    .rsplit('/')
                    .next()
                    .unwrap()
            })
            .collect();
        if !files.iter().any(|file| names.contains(*file)) {
            missing.push(format!("{} {}", api["api_id"], files.join(" ")));
        }
    }
    assert!(missing.is_empty(), "not implemented: {missing:?}");
    assert_eq!(snapshot()["apis"].as_array().unwrap().len(), 85);
}
