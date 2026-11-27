//! The registration statement types, one `Vec` per group of the reply.

use super::groups::{FromGroups, Group, take};
use super::rows::{
    BondGeneral, FundUse, IssuedSecurity, OfferedSecurity, OfferingGeneral, PartyCompany,
    PutBackRight, RestructuringGeneral, Seller, Underwriter,
};
use crate::Error;

/// An equity (지분증권) or depositary receipt (증권예탁증권) registration statement,
/// grouped as in the reply.
///
/// Each group holds the rows of every statement filed in the date range;
/// tell them apart by `rcept_no`. A group the reply leaves out is empty.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct EquityRegistration {
    /// 일반사항
    pub general: Vec<OfferingGeneral>,
    /// 증권의종류
    pub securities: Vec<OfferedSecurity>,
    /// 인수인정보
    pub underwriters: Vec<Underwriter>,
    /// 자금의사용목적
    pub fund_uses: Vec<FundUse>,
    /// 매출인에관한사항
    pub sellers: Vec<Seller>,
    /// 일반청약자환매청구권
    pub put_back_rights: Vec<PutBackRight>,
}

impl FromGroups for EquityRegistration {
    fn from_groups(endpoint: &'static str, groups: &mut [Group]) -> Result<Self, Error> {
        Ok(EquityRegistration {
            general: take(endpoint, groups, "일반사항")?,
            securities: take(endpoint, groups, "증권의종류")?,
            underwriters: take(endpoint, groups, "인수인정보")?,
            fund_uses: take(endpoint, groups, "자금의사용목적")?,
            sellers: take(endpoint, groups, "매출인에관한사항")?,
            put_back_rights: take(endpoint, groups, "일반청약자환매청구권")?,
        })
    }
}

/// A debt securities (채무증권) registration statement, grouped as in the reply.
///
/// Each group holds the rows of every statement filed in the date range;
/// tell them apart by `rcept_no`. A group the reply leaves out is empty.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct DebtRegistration {
    /// 일반사항
    pub general: Vec<BondGeneral>,
    /// 인수인정보
    pub underwriters: Vec<Underwriter>,
    /// 자금의사용목적
    pub fund_uses: Vec<FundUse>,
    /// 매출인에관한사항
    pub sellers: Vec<Seller>,
}

impl FromGroups for DebtRegistration {
    fn from_groups(endpoint: &'static str, groups: &mut [Group]) -> Result<Self, Error> {
        Ok(DebtRegistration {
            general: take(endpoint, groups, "일반사항")?,
            underwriters: take(endpoint, groups, "인수인정보")?,
            fund_uses: take(endpoint, groups, "자금의사용목적")?,
            sellers: take(endpoint, groups, "매출인에관한사항")?,
        })
    }
}

/// A merger (합병), share exchange or transfer (주식의포괄적교환·이전) or split (분할)
/// registration statement, grouped as in the reply.
///
/// Each group holds the rows of every statement filed in the date range;
/// tell them apart by `rcept_no`. A group the reply leaves out is empty.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct RestructuringRegistration {
    /// 일반사항
    pub general: Vec<RestructuringGeneral>,
    /// 발행증권
    pub securities: Vec<IssuedSecurity>,
    /// 당사회사에관한사항
    pub companies: Vec<PartyCompany>,
}

impl FromGroups for RestructuringRegistration {
    fn from_groups(endpoint: &'static str, groups: &mut [Group]) -> Result<Self, Error> {
        Ok(RestructuringRegistration {
            general: take(endpoint, groups, "일반사항")?,
            securities: take(endpoint, groups, "발행증권")?,
            companies: take(endpoint, groups, "당사회사에관한사항")?,
        })
    }
}
