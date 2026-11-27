use serde::Deserialize;

/// The market a company is listed on (`corp_cls`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(from = "String")]
#[non_exhaustive]
pub enum CorpClass {
    /// `Y`: KOSPI (유가증권시장).
    Kospi,
    /// `K`: KOSDAQ (코스닥).
    Kosdaq,
    /// `N`: KONEX (코넥스).
    Konex,
    /// `E`: every other company (기타법인), including unlisted ones.
    Etc,
    /// A code this crate does not know yet.
    Unknown(String),
}

impl CorpClass {
    /// The raw one-letter code, e.g. `"Y"`.
    pub fn code(&self) -> &str {
        match self {
            CorpClass::Kospi => "Y",
            CorpClass::Kosdaq => "K",
            CorpClass::Konex => "N",
            CorpClass::Etc => "E",
            CorpClass::Unknown(code) => code,
        }
    }
}

impl From<String> for CorpClass {
    fn from(code: String) -> Self {
        match code.as_str() {
            "Y" => CorpClass::Kospi,
            "K" => CorpClass::Kosdaq,
            "N" => CorpClass::Konex,
            "E" => CorpClass::Etc,
            _ => CorpClass::Unknown(code),
        }
    }
}
