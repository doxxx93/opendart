//! The registration statement types: one value per statement, with the
//! rows of each group of the reply that carry its receipt number.

use super::groups::{Group, Statement, statement, take};
use super::rows::{
    BondGeneral, FundUse, IssuedSecurity, OfferedSecurity, OfferingGeneral, PartyCompany,
    PutBackRight, RestructuringGeneral, Seller, Underwriter,
};
use crate::Error;

/// One equity (지분증권) or depositary receipt (증권예탁증권) registration statement.
///
/// Each field holds the rows of one group of the reply (named in its
/// doc) that belong to this statement. A group the reply leaves out is
/// empty.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct EquityRegistration {
    /// 접수번호: the 14-digit receipt number of the statement.
    pub rcept_no: String,
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

impl Statement for EquityRegistration {
    fn new(rcept_no: &str) -> Self {
        EquityRegistration {
            rcept_no: rcept_no.to_owned(),
            general: Vec::new(),
            securities: Vec::new(),
            underwriters: Vec::new(),
            fund_uses: Vec::new(),
            sellers: Vec::new(),
            put_back_rights: Vec::new(),
        }
    }

    fn rcept_no(&self) -> &str {
        &self.rcept_no
    }

    fn from_groups(endpoint: &'static str, groups: &mut [Group]) -> Result<Vec<Self>, Error> {
        let mut statements: Vec<Self> = Vec::new();
        for row in take::<OfferingGeneral>(endpoint, groups, "일반사항")? {
            statement(&mut statements, &row.rcept_no).general.push(row);
        }
        for row in take::<OfferedSecurity>(endpoint, groups, "증권의종류")? {
            statement(&mut statements, &row.rcept_no)
                .securities
                .push(row);
        }
        for row in take::<Underwriter>(endpoint, groups, "인수인정보")? {
            statement(&mut statements, &row.rcept_no)
                .underwriters
                .push(row);
        }
        for row in take::<FundUse>(endpoint, groups, "자금의사용목적")? {
            statement(&mut statements, &row.rcept_no)
                .fund_uses
                .push(row);
        }
        for row in take::<Seller>(endpoint, groups, "매출인에관한사항")? {
            statement(&mut statements, &row.rcept_no).sellers.push(row);
        }
        for row in take::<PutBackRight>(endpoint, groups, "일반청약자환매청구권")? {
            statement(&mut statements, &row.rcept_no)
                .put_back_rights
                .push(row);
        }
        Ok(statements)
    }
}

/// One debt securities (채무증권) registration statement.
///
/// Each field holds the rows of one group of the reply (named in its
/// doc) that belong to this statement. A group the reply leaves out is
/// empty.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct DebtRegistration {
    /// 접수번호: the 14-digit receipt number of the statement.
    pub rcept_no: String,
    /// 일반사항
    pub general: Vec<BondGeneral>,
    /// 인수인정보
    pub underwriters: Vec<Underwriter>,
    /// 자금의사용목적
    pub fund_uses: Vec<FundUse>,
    /// 매출인에관한사항
    pub sellers: Vec<Seller>,
}

impl Statement for DebtRegistration {
    fn new(rcept_no: &str) -> Self {
        DebtRegistration {
            rcept_no: rcept_no.to_owned(),
            general: Vec::new(),
            underwriters: Vec::new(),
            fund_uses: Vec::new(),
            sellers: Vec::new(),
        }
    }

    fn rcept_no(&self) -> &str {
        &self.rcept_no
    }

    fn from_groups(endpoint: &'static str, groups: &mut [Group]) -> Result<Vec<Self>, Error> {
        let mut statements: Vec<Self> = Vec::new();
        for row in take::<BondGeneral>(endpoint, groups, "일반사항")? {
            statement(&mut statements, &row.rcept_no).general.push(row);
        }
        for row in take::<Underwriter>(endpoint, groups, "인수인정보")? {
            statement(&mut statements, &row.rcept_no)
                .underwriters
                .push(row);
        }
        for row in take::<FundUse>(endpoint, groups, "자금의사용목적")? {
            statement(&mut statements, &row.rcept_no)
                .fund_uses
                .push(row);
        }
        for row in take::<Seller>(endpoint, groups, "매출인에관한사항")? {
            statement(&mut statements, &row.rcept_no).sellers.push(row);
        }
        Ok(statements)
    }
}

/// One merger (합병), share exchange or transfer (주식의포괄적교환·이전) or split (분할)
/// registration statement.
///
/// Each field holds the rows of one group of the reply (named in its
/// doc) that belong to this statement. A group the reply leaves out is
/// empty.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct RestructuringRegistration {
    /// 접수번호: the 14-digit receipt number of the statement.
    pub rcept_no: String,
    /// 일반사항
    pub general: Vec<RestructuringGeneral>,
    /// 발행증권
    pub securities: Vec<IssuedSecurity>,
    /// 당사회사에관한사항
    pub companies: Vec<PartyCompany>,
}

impl Statement for RestructuringRegistration {
    fn new(rcept_no: &str) -> Self {
        RestructuringRegistration {
            rcept_no: rcept_no.to_owned(),
            general: Vec::new(),
            securities: Vec::new(),
            companies: Vec::new(),
        }
    }

    fn rcept_no(&self) -> &str {
        &self.rcept_no
    }

    fn from_groups(endpoint: &'static str, groups: &mut [Group]) -> Result<Vec<Self>, Error> {
        let mut statements: Vec<Self> = Vec::new();
        for row in take::<RestructuringGeneral>(endpoint, groups, "일반사항")? {
            statement(&mut statements, &row.rcept_no).general.push(row);
        }
        for row in take::<IssuedSecurity>(endpoint, groups, "발행증권")? {
            statement(&mut statements, &row.rcept_no)
                .securities
                .push(row);
        }
        for row in take::<PartyCompany>(endpoint, groups, "당사회사에관한사항")? {
            statement(&mut statements, &row.rcept_no)
                .companies
                .push(row);
        }
        Ok(statements)
    }
}
