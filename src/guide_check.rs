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

/// A `Deserializer` that records the field names serde's derive passes to
/// `deserialize_struct`, then fails.
struct FieldProbe<'a>(&'a mut Option<&'static [&'static str]>);

impl<'de> serde::Deserializer<'de> for FieldProbe<'_> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V: serde::de::Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        Err(serde::de::Error::custom("not a struct"))
    }

    fn deserialize_struct<V: serde::de::Visitor<'de>>(
        self,
        _: &'static str,
        fields: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error> {
        *self.0 = Some(fields);
        Err(serde::de::Error::custom("probe"))
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf
        option unit unit_struct newtype_struct seq tuple tuple_struct map enum identifier
        ignored_any
    }
}

/// The wire names of the fields of `T`.
fn fields<T: serde::de::DeserializeOwned>() -> &'static [&'static str] {
    let mut names = None;
    let _ = T::deserialize(FieldProbe(&mut names));
    names.expect("not a struct")
}

/// When the guide adds a response key, this names the endpoint whose
/// struct lacks the field (the guide watch only reports that the guide
/// changed).
#[test]
fn every_guide_response_key_is_a_field() {
    use crate::{disclosure, financial, major_event, ownership, periodic, registration};
    let types: Vec<(&str, Vec<&[&str]>)> = vec![
        ("company.json", vec![fields::<disclosure::Company>()]),
        ("corpCode.xml", vec![fields::<disclosure::CorpCodeEntry>()]),
        ("document.xml", vec![]),
        (
            "list.json",
            vec![
                fields::<disclosure::DisclosurePage>(),
                fields::<disclosure::Disclosure>(),
            ],
        ),
        (
            "fnlttCmpnyIndx.json",
            vec![fields::<financial::Indicator>()],
        ),
        ("fnlttMultiAcnt.json", vec![fields::<financial::Account>()]),
        ("fnlttSinglAcnt.json", vec![fields::<financial::Account>()]),
        (
            "fnlttSinglAcntAll.json",
            vec![fields::<financial::StatementLine>()],
        ),
        (
            "fnlttSinglIndx.json",
            vec![fields::<financial::Indicator>()],
        ),
        ("fnlttXbrl.xml", vec![]),
        (
            "xbrlTaxonomy.json",
            vec![fields::<financial::TaxonomyAccount>()],
        ),
        (
            "astInhtrfEtcPtbkOpt.json",
            vec![fields::<major_event::AssetTransferPutBack>()],
        ),
        (
            "bdwtIsDecsn.json",
            vec![fields::<major_event::BondWithWarrantsIssue>()],
        ),
        (
            "bnkMngtPcbg.json",
            vec![fields::<major_event::CreditorManagementStart>()],
        ),
        (
            "bnkMngtPcsp.json",
            vec![fields::<major_event::CreditorManagementEnd>()],
        ),
        (
            "bsnInhDecsn.json",
            vec![fields::<major_event::BusinessAcquisition>()],
        ),
        (
            "bsnSp.json",
            vec![fields::<major_event::BusinessSuspension>()],
        ),
        (
            "bsnTrfDecsn.json",
            vec![fields::<major_event::BusinessTransfer>()],
        ),
        (
            "cmpDvDecsn.json",
            vec![fields::<major_event::CorporateSplit>()],
        ),
        (
            "cmpDvmgDecsn.json",
            vec![fields::<major_event::SplitMerger>()],
        ),
        ("cmpMgDecsn.json", vec![fields::<major_event::Merger>()]),
        (
            "crDecsn.json",
            vec![fields::<major_event::CapitalReduction>()],
        ),
        (
            "ctrcvsBgrq.json",
            vec![fields::<major_event::RehabilitationFiling>()],
        ),
        (
            "cvbdIsDecsn.json",
            vec![fields::<major_event::ConvertibleBondIssue>()],
        ),
        ("dfOcr.json", vec![fields::<major_event::DefaultEvent>()]),
        (
            "dsRsOcr.json",
            vec![fields::<major_event::DissolutionCause>()],
        ),
        (
            "exbdIsDecsn.json",
            vec![fields::<major_event::ExchangeableBondIssue>()],
        ),
        ("fricDecsn.json", vec![fields::<major_event::BonusIssue>()]),
        ("lwstLg.json", vec![fields::<major_event::Lawsuit>()]),
        (
            "otcprStkInvscrInhDecsn.json",
            vec![fields::<major_event::OtherCorpStockAcquisition>()],
        ),
        (
            "otcprStkInvscrTrfDecsn.json",
            vec![fields::<major_event::OtherCorpStockTransfer>()],
        ),
        (
            "ovDlst.json",
            vec![fields::<major_event::OverseasDelisting>()],
        ),
        (
            "ovDlstDecsn.json",
            vec![fields::<major_event::OverseasDelistingDecision>()],
        ),
        ("ovLst.json", vec![fields::<major_event::OverseasListing>()]),
        (
            "ovLstDecsn.json",
            vec![fields::<major_event::OverseasListingDecision>()],
        ),
        (
            "pifricDecsn.json",
            vec![fields::<major_event::PaidInAndBonusIssue>()],
        ),
        (
            "piicDecsn.json",
            vec![fields::<major_event::PaidInCapitalIncrease>()],
        ),
        (
            "stkExtrDecsn.json",
            vec![fields::<major_event::ShareExchange>()],
        ),
        (
            "stkrtbdInhDecsn.json",
            vec![fields::<major_event::EquityLinkedBondAcquisition>()],
        ),
        (
            "stkrtbdTrfDecsn.json",
            vec![fields::<major_event::EquityLinkedBondTransfer>()],
        ),
        (
            "tgastInhDecsn.json",
            vec![fields::<major_event::TangibleAssetAcquisition>()],
        ),
        (
            "tgastTrfDecsn.json",
            vec![fields::<major_event::TangibleAssetTransfer>()],
        ),
        (
            "tsstkAqDecsn.json",
            vec![fields::<major_event::TreasuryStockAcquisition>()],
        ),
        (
            "tsstkAqTrctrCcDecsn.json",
            vec![fields::<major_event::TreasuryTrustTermination>()],
        ),
        (
            "tsstkAqTrctrCnsDecsn.json",
            vec![fields::<major_event::TreasuryTrustContract>()],
        ),
        (
            "tsstkDpDecsn.json",
            vec![fields::<major_event::TreasuryStockDisposal>()],
        ),
        (
            "wdCocobdIsDecsn.json",
            vec![fields::<major_event::WriteDownContingentBondIssue>()],
        ),
        ("elestock.json", vec![fields::<ownership::InsiderHolding>()]),
        ("majorstock.json", vec![fields::<ownership::MajorHolding>()]),
        (
            "accnutAdtorNmNdAdtOpinion.json",
            vec![fields::<periodic::AuditOpinion>()],
        ),
        (
            "accnutAdtorNonAdtServcCnclsSttus.json",
            vec![fields::<periodic::NonAuditContract>()],
        ),
        (
            "adtServcCnclsSttus.json",
            vec![fields::<periodic::AuditContract>()],
        ),
        ("alotMatter.json", vec![fields::<periodic::Dividend>()]),
        (
            "cndlCaplScritsNrdmpBlce.json",
            vec![fields::<periodic::ContingentCapitalBalance>()],
        ),
        (
            "cprndNrdmpBlce.json",
            vec![fields::<periodic::CorporateBondBalance>()],
        ),
        (
            "detScritsIsuAcmslt.json",
            vec![fields::<periodic::DebtIssuance>()],
        ),
        (
            "drctrAdtAllMendngSttusGmtsckConfmAmount.json",
            vec![fields::<periodic::ApprovedDirectorPay>()],
        ),
        (
            "drctrAdtAllMendngSttusMendngPymntamtTyCl.json",
            vec![fields::<periodic::DirectorPayByType>()],
        ),
        ("empSttus.json", vec![fields::<periodic::EmployeeCount>()]),
        (
            "entrprsBilScritsNrdmpBlce.json",
            vec![fields::<periodic::CommercialPaperBalance>()],
        ),
        ("exctvSttus.json", vec![fields::<periodic::Executive>()]),
        (
            "hmvAuditAllSttus.json",
            vec![fields::<periodic::DirectorPayTotal>()],
        ),
        (
            "hmvAuditIndvdlBySttus.json",
            vec![fields::<periodic::IndividualPay>()],
        ),
        (
            "hmvAuditIndvdlBySttusV2.json",
            vec![
                fields::<periodic::PayReport>(),
                fields::<periodic::PayRecipient>(),
                fields::<periodic::StockBasedPay>(),
            ],
        ),
        (
            "hyslrChgSttus.json",
            vec![fields::<periodic::LargestShareholderChange>()],
        ),
        (
            "hyslrSttus.json",
            vec![fields::<periodic::LargestShareholder>()],
        ),
        (
            "indvdlByPay.json",
            vec![fields::<periodic::IndividualPay>()],
        ),
        (
            "indvdlByPayV2.json",
            vec![
                fields::<periodic::PayReport>(),
                fields::<periodic::PayRecipient>(),
                fields::<periodic::StockBasedPay>(),
            ],
        ),
        ("irdsSttus.json", vec![fields::<periodic::CapitalChange>()]),
        (
            "mrhlSttus.json",
            vec![fields::<periodic::MinorityShareholding>()],
        ),
        (
            "newCaplScritsNrdmpBlce.json",
            vec![fields::<periodic::HybridSecuritiesBalance>()],
        ),
        (
            "otrCprInvstmntSttus.json",
            vec![fields::<periodic::Investment>()],
        ),
        (
            "outcmpnyDrctrNdChangeSttus.json",
            vec![fields::<periodic::OutsideDirectorCount>()],
        ),
        (
            "prvsrpCptalUseDtls.json",
            vec![fields::<periodic::PrivatePlacementFundUse>()],
        ),
        (
            "pssrpCptalUseDtls.json",
            vec![fields::<periodic::PublicOfferingFundUse>()],
        ),
        (
            "srtpdPsndbtNrdmpBlce.json",
            vec![fields::<periodic::ShortTermBondBalance>()],
        ),
        (
            "stockTotqySttus.json",
            vec![fields::<periodic::ShareCount>()],
        ),
        (
            "tesstkAcqsDspsSttus.json",
            vec![fields::<periodic::TreasuryStock>()],
        ),
        (
            "unrstExctvMendngSttus.json",
            vec![fields::<periodic::UnregisteredExecutivePay>()],
        ),
        (
            "bdRs.json",
            vec![
                fields::<registration::BondGeneral>(),
                fields::<registration::Underwriter>(),
                fields::<registration::FundUse>(),
                fields::<registration::Seller>(),
            ],
        ),
        (
            "dvRs.json",
            vec![
                fields::<registration::RestructuringGeneral>(),
                fields::<registration::IssuedSecurity>(),
                fields::<registration::PartyCompany>(),
            ],
        ),
        (
            "estkRs.json",
            vec![
                fields::<registration::OfferingGeneral>(),
                fields::<registration::OfferedSecurity>(),
                fields::<registration::Underwriter>(),
                fields::<registration::FundUse>(),
                fields::<registration::Seller>(),
                fields::<registration::PutBackRight>(),
            ],
        ),
        (
            "extrRs.json",
            vec![
                fields::<registration::RestructuringGeneral>(),
                fields::<registration::IssuedSecurity>(),
                fields::<registration::PartyCompany>(),
            ],
        ),
        (
            "mgRs.json",
            vec![
                fields::<registration::RestructuringGeneral>(),
                fields::<registration::IssuedSecurity>(),
                fields::<registration::PartyCompany>(),
            ],
        ),
        (
            "stkdpRs.json",
            vec![
                fields::<registration::OfferingGeneral>(),
                fields::<registration::OfferedSecurity>(),
                fields::<registration::Underwriter>(),
                fields::<registration::FundUse>(),
                fields::<registration::Seller>(),
                fields::<registration::PutBackRight>(),
            ],
        ),
    ];
    assert_eq!(types.len(), 85);
    let mut missing = Vec::new();
    for (endpoint, structs) in &types {
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
        for key in api["response"].as_array().unwrap() {
            let key = key["key"].as_str().unwrap();
            let envelope = ["result", "status", "message", "list", "group", "title"].contains(&key);
            if !envelope && !structs.iter().any(|names| names.contains(&key)) {
                missing.push(format!("{endpoint} {key}"));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "guide keys without a field: {missing:?}"
    );
}
