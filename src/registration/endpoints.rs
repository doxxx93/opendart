//! The six 증권신고서 endpoints.

use super::types::{DebtRegistration, EquityRegistration, RestructuringRegistration};
use crate::{Client, DateRangeRequest, Error};

const ESTK_RS: &str = "estkRs.json";
const BD_RS: &str = "bdRs.json";
const STKDP_RS: &str = "stkdpRs.json";
const MG_RS: &str = "mgRs.json";
const EXTR_RS: &str = "extrRs.json";
const DV_RS: &str = "dvRs.json";

impl Client {
    /// 지분증권 (`estkRs.json`): equity securities registration statements filed in
    /// the date range, one per
    /// statement.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn estk_rs(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<EquityRegistration>, Error> {
        self.registration(ESTK_RS, request).await
    }

    /// 채무증권 (`bdRs.json`): debt securities registration statements filed in the
    /// date range, one per
    /// statement.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn bd_rs(&self, request: &DateRangeRequest) -> Result<Vec<DebtRegistration>, Error> {
        self.registration(BD_RS, request).await
    }

    /// 증권예탁증권 (`stkdpRs.json`): depositary receipt registration statements
    /// filed in the date range, one per
    /// statement.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn stkdp_rs(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<EquityRegistration>, Error> {
        self.registration(STKDP_RS, request).await
    }

    /// 합병 (`mgRs.json`): merger registration statements filed in the date
    /// range, one per
    /// statement.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn mg_rs(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<RestructuringRegistration>, Error> {
        self.registration(MG_RS, request).await
    }

    /// 주식의포괄적교환·이전 (`extrRs.json`): share exchange and transfer registration
    /// statements filed in the date range, one per
    /// statement.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn extr_rs(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<RestructuringRegistration>, Error> {
        self.registration(EXTR_RS, request).await
    }

    /// 분할 (`dvRs.json`): split registration statements filed in the date range,
    /// one per
    /// statement.
    ///
    /// Returns an empty list when OpenDART has no data for the request
    /// ([`Status::NoData`](crate::Status::NoData)); see the
    /// [crate docs](crate#no-data).
    pub async fn dv_rs(
        &self,
        request: &DateRangeRequest,
    ) -> Result<Vec<RestructuringRegistration>, Error> {
        self.registration(DV_RS, request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registration::groups::parse;

    #[test]
    fn decodes_estk_rs() {
        let body = include_bytes!("../../tests/fixtures/registration/estkRs.json");
        let statements: Vec<EquityRegistration> = parse(ESTK_RS, body).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(statements[0].rcept_no, "20190401004959");
        assert_eq!(statements[0].general.len(), 1);
        assert_eq!(statements[0].securities.len(), 1);
        assert_eq!(statements[0].underwriters.len(), 1);
        assert_eq!(statements[0].fund_uses.len(), 2);
        assert_eq!(statements[0].sellers.len(), 1);
        assert_eq!(statements[0].put_back_rights.len(), 1);
        crate::guide_check::assert_documented(ESTK_RS, &crate::date_range::example().query());
    }

    #[test]
    fn decodes_bd_rs() {
        let body = include_bytes!("../../tests/fixtures/registration/bdRs.json");
        let statements: Vec<DebtRegistration> = parse(BD_RS, body).unwrap();
        assert_eq!(statements.len(), 2);
        assert_eq!(statements[0].rcept_no, "20190214000140");
        assert_eq!(statements[0].general.len(), 1);
        assert_eq!(statements[0].underwriters.len(), 2);
        assert_eq!(statements[0].fund_uses.len(), 2);
        assert_eq!(statements[0].sellers.len(), 2);
        crate::guide_check::assert_documented(BD_RS, &crate::date_range::example().query());
    }

    #[test]
    fn decodes_stkdp_rs() {
        let body = include_bytes!("../../tests/fixtures/registration/stkdpRs.json");
        let statements: Vec<EquityRegistration> = parse(STKDP_RS, body).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(statements[0].rcept_no, "20190411002251");
        assert_eq!(statements[0].general.len(), 1);
        assert_eq!(statements[0].securities.len(), 1);
        assert_eq!(statements[0].underwriters.len(), 2);
        assert_eq!(statements[0].fund_uses.len(), 2);
        assert_eq!(statements[0].sellers.len(), 1);
        assert_eq!(statements[0].put_back_rights.len(), 0);
        crate::guide_check::assert_documented(STKDP_RS, &crate::date_range::example().query());
    }

    #[test]
    fn decodes_mg_rs() {
        let body = include_bytes!("../../tests/fixtures/registration/mgRs.json");
        let statements: Vec<RestructuringRegistration> = parse(MG_RS, body).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(statements[0].rcept_no, "20190402002153");
        assert_eq!(statements[0].general.len(), 1);
        assert_eq!(statements[0].securities.len(), 1);
        assert_eq!(statements[0].companies.len(), 2);
        crate::guide_check::assert_documented(MG_RS, &crate::date_range::example().query());
    }

    #[test]
    fn decodes_extr_rs() {
        let body = include_bytes!("../../tests/fixtures/registration/extrRs.json");
        let statements: Vec<RestructuringRegistration> = parse(EXTR_RS, body).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(statements[0].rcept_no, "20190417000436");
        assert_eq!(statements[0].general.len(), 1);
        assert_eq!(statements[0].securities.len(), 1);
        assert_eq!(statements[0].companies.len(), 2);
        crate::guide_check::assert_documented(EXTR_RS, &crate::date_range::example().query());
    }

    #[test]
    fn decodes_dv_rs() {
        let body = include_bytes!("../../tests/fixtures/registration/dvRs.json");
        let statements: Vec<RestructuringRegistration> = parse(DV_RS, body).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(statements[0].rcept_no, "20191015000270");
        assert_eq!(statements[0].general.len(), 1);
        assert_eq!(statements[0].securities.len(), 1);
        assert_eq!(statements[0].companies.len(), 2);
        crate::guide_check::assert_documented(DV_RS, &crate::date_range::example().query());
    }
}
