//! 정기보고서 주요정보 (key information in periodic reports), OpenDART
//! group `DS002`.
//!
//! Every endpoint takes a [`ReportRequest`](crate::ReportRequest): one
//! company's annual, half-year or quarterly report. Most return one row per
//! line of the corresponding table in the report, and an empty list when
//! the report has no such table. Numeric columns are
//! [`Number`](crate::Number)s, parsed by the caller.
//!
//! ```no_run
//! use opendart::{Client, ReportCode, ReportRequest};
//!
//! # async fn run(dart: Client) -> Result<(), opendart::Error> {
//! let request = ReportRequest::new("00126380", 2025, ReportCode::Annual);
//! for row in dart.alot_matter(&request).await? {
//!     println!("{} {}: {:?}", row.se, row.stock_knd, row.thstrm);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! | Endpoint | Method | Row | 개발가이드 |
//! |---|---|---|---|
//! | `irdsSttus.json` | [`Client::irds_sttus`] | [`CapitalChange`] | [증자(감자) 현황](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2019004) |
//! | `alotMatter.json` | [`Client::alot_matter`] | [`Dividend`] | [배당에 관한 사항](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2019005) |
//! | `tesstkAcqsDspsSttus.json` | [`Client::tesstk_acqs_dsps_sttus`] | [`TreasuryStock`] | [자기주식 취득 및 처분 현황](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2019006) |
//! | `hyslrSttus.json` | [`Client::hyslr_sttus`] | [`LargestShareholder`] | [최대주주 현황](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2019007) |
//! | `hyslrChgSttus.json` | [`Client::hyslr_chg_sttus`] | [`LargestShareholderChange`] | [최대주주 변동현황](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2019008) |
//! | `mrhlSttus.json` | [`Client::mrhl_sttus`] | [`MinorityShareholders`] | [소액주주 현황](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2019009) |
//! | `exctvSttus.json` | [`Client::exctv_sttus`] | [`Executive`] | [임원 현황](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2019010) |
//! | `empSttus.json` | [`Client::emp_sttus`] | [`Employees`] | [직원 현황](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2019011) |
//! | `hmvAuditIndvdlBySttus.json` | [`Client::hmv_audit_indvdl_by_sttus`] | [`IndividualPay`] | [이사·감사의 개인별 보수현황(5억원 이상)](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2019012) |
//! | `hmvAuditAllSttus.json` | [`Client::hmv_audit_all_sttus`] | [`DirectorPayTotal`] | [이사·감사 전체의 보수현황(보수지급금액 - 이사·감사 전체)](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2019013) |
//! | `indvdlByPay.json` | [`Client::indvdl_by_pay`] | [`IndividualPay`] | [개인별 보수지급 금액(5억이상 상위5인)](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2019014) |
//! | `otrCprInvstmntSttus.json` | [`Client::otr_cpr_invstmnt_sttus`] | [`Investment`] | [타법인 출자현황](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2019015) |
//! | `stockTotqySttus.json` | [`Client::stock_totqy_sttus`] | [`ShareCount`] | [주식의 총수 현황](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020002) |
//! | `detScritsIsuAcmslt.json` | [`Client::det_scrits_isu_acmslt`] | [`DebtIssuance`] | [채무증권 발행실적](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020003) |
//! | `entrprsBilScritsNrdmpBlce.json` | [`Client::entrprs_bil_scrits_nrdmp_blce`] | [`CommercialPaperBalance`] | [기업어음증권 미상환 잔액](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020004) |
//! | `srtpdPsndbtNrdmpBlce.json` | [`Client::srtpd_psndbt_nrdmp_blce`] | [`ShortTermBondBalance`] | [단기사채 미상환 잔액](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020005) |
//! | `cprndNrdmpBlce.json` | [`Client::cprnd_nrdmp_blce`] | [`CorporateBondBalance`] | [회사채 미상환 잔액](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020006) |
//! | `newCaplScritsNrdmpBlce.json` | [`Client::new_capl_scrits_nrdmp_blce`] | [`HybridSecuritiesBalance`] | [신종자본증권 미상환 잔액](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020007) |
//! | `cndlCaplScritsNrdmpBlce.json` | [`Client::cndl_capl_scrits_nrdmp_blce`] | [`ContingentCapitalBalance`] | [조건부 자본증권 미상환 잔액](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020008) |
//! | `accnutAdtorNmNdAdtOpinion.json` | [`Client::accnut_adtor_nm_nd_adt_opinion`] | [`AuditOpinion`] | [회계감사인의 명칭 및 감사의견](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020009) |
//! | `adtServcCnclsSttus.json` | [`Client::adt_servc_cncls_sttus`] | [`AuditContract`] | [감사용역체결현황](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020010) |
//! | `accnutAdtorNonAdtServcCnclsSttus.json` | [`Client::accnut_adtor_non_adt_servc_cncls_sttus`] | [`NonAuditContract`] | [회계감사인과의 비감사용역 계약체결 현황](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020011) |
//! | `outcmpnyDrctrNdChangeSttus.json` | [`Client::outcmpny_drctr_nd_change_sttus`] | [`OutsideDirectors`] | [독립(사외)이사 및 그 변동현황](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020012) |
//! | `unrstExctvMendngSttus.json` | [`Client::unrst_exctv_mendng_sttus`] | [`UnregisteredExecutivePay`] | [미등기임원 보수현황](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020013) |
//! | `drctrAdtAllMendngSttusGmtsckConfmAmount.json` | [`Client::drctr_adt_all_mendng_sttus_gmtsck_confm_amount`] | [`ApprovedDirectorPay`] | [이사·감사 전체의 보수현황(주주총회 승인금액)](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020014) |
//! | `drctrAdtAllMendngSttusMendngPymntamtTyCl.json` | [`Client::drctr_adt_all_mendng_sttus_mendng_pymntamt_ty_cl`] | [`DirectorPayByType`] | [이사·감사 전체의 보수현황(보수지급금액 - 유형별)](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020015) |
//! | `pssrpCptalUseDtls.json` | [`Client::pssrp_cptal_use_dtls`] | [`PublicOfferingFundUse`] | [공모자금의 사용내역](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020016) |
//! | `prvsrpCptalUseDtls.json` | [`Client::prvsrp_cptal_use_dtls`] | [`PrivatePlacementFundUse`] | [사모자금의 사용내역](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2020017) |
//! | `hmvAuditIndvdlBySttusV2.json` | [`Client::hmv_audit_indvdl_by_sttus_v2`] | [`PayReport`] | [이사·감사의 개인별 보수현황(5억원 이상) (Ver 2.0)](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2026001) |
//! | `indvdlByPayV2.json` | [`Client::indvdl_by_pay_v2`] | [`PayReport`] | [개인별 보수지급 금액(5억이상 상위5인) (Ver 2.0)](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS002&apiId=2026002) |
//!
//! [`Client::irds_sttus`]: crate::Client::irds_sttus
//! [`Client::alot_matter`]: crate::Client::alot_matter
//! [`Client::tesstk_acqs_dsps_sttus`]: crate::Client::tesstk_acqs_dsps_sttus
//! [`Client::hyslr_sttus`]: crate::Client::hyslr_sttus
//! [`Client::hyslr_chg_sttus`]: crate::Client::hyslr_chg_sttus
//! [`Client::mrhl_sttus`]: crate::Client::mrhl_sttus
//! [`Client::exctv_sttus`]: crate::Client::exctv_sttus
//! [`Client::emp_sttus`]: crate::Client::emp_sttus
//! [`Client::hmv_audit_indvdl_by_sttus`]: crate::Client::hmv_audit_indvdl_by_sttus
//! [`Client::hmv_audit_all_sttus`]: crate::Client::hmv_audit_all_sttus
//! [`Client::indvdl_by_pay`]: crate::Client::indvdl_by_pay
//! [`Client::otr_cpr_invstmnt_sttus`]: crate::Client::otr_cpr_invstmnt_sttus
//! [`Client::stock_totqy_sttus`]: crate::Client::stock_totqy_sttus
//! [`Client::det_scrits_isu_acmslt`]: crate::Client::det_scrits_isu_acmslt
//! [`Client::entrprs_bil_scrits_nrdmp_blce`]: crate::Client::entrprs_bil_scrits_nrdmp_blce
//! [`Client::srtpd_psndbt_nrdmp_blce`]: crate::Client::srtpd_psndbt_nrdmp_blce
//! [`Client::cprnd_nrdmp_blce`]: crate::Client::cprnd_nrdmp_blce
//! [`Client::new_capl_scrits_nrdmp_blce`]: crate::Client::new_capl_scrits_nrdmp_blce
//! [`Client::cndl_capl_scrits_nrdmp_blce`]: crate::Client::cndl_capl_scrits_nrdmp_blce
//! [`Client::accnut_adtor_nm_nd_adt_opinion`]: crate::Client::accnut_adtor_nm_nd_adt_opinion
//! [`Client::adt_servc_cncls_sttus`]: crate::Client::adt_servc_cncls_sttus
//! [`Client::accnut_adtor_non_adt_servc_cncls_sttus`]: crate::Client::accnut_adtor_non_adt_servc_cncls_sttus
//! [`Client::outcmpny_drctr_nd_change_sttus`]: crate::Client::outcmpny_drctr_nd_change_sttus
//! [`Client::unrst_exctv_mendng_sttus`]: crate::Client::unrst_exctv_mendng_sttus
//! [`Client::drctr_adt_all_mendng_sttus_gmtsck_confm_amount`]: crate::Client::drctr_adt_all_mendng_sttus_gmtsck_confm_amount
//! [`Client::drctr_adt_all_mendng_sttus_mendng_pymntamt_ty_cl`]: crate::Client::drctr_adt_all_mendng_sttus_mendng_pymntamt_ty_cl
//! [`Client::pssrp_cptal_use_dtls`]: crate::Client::pssrp_cptal_use_dtls
//! [`Client::prvsrp_cptal_use_dtls`]: crate::Client::prvsrp_cptal_use_dtls
//! [`Client::hmv_audit_indvdl_by_sttus_v2`]: crate::Client::hmv_audit_indvdl_by_sttus_v2
//! [`Client::indvdl_by_pay_v2`]: crate::Client::indvdl_by_pay_v2

mod accnut_adtor_nm_nd_adt_opinion;
mod accnut_adtor_non_adt_servc_cncls_sttus;
mod adt_servc_cncls_sttus;
mod alot_matter;
mod cndl_capl_scrits_nrdmp_blce;
mod cprnd_nrdmp_blce;
mod det_scrits_isu_acmslt;
mod drctr_adt_all_mendng_sttus_gmtsck_confm_amount;
mod drctr_adt_all_mendng_sttus_mendng_pymntamt_ty_cl;
mod emp_sttus;
mod entrprs_bil_scrits_nrdmp_blce;
mod exctv_sttus;
mod hmv_audit_all_sttus;
mod hmv_audit_indvdl_by_sttus;
mod hyslr_chg_sttus;
mod hyslr_sttus;
mod indvdl_by_pay;
mod irds_sttus;
mod mrhl_sttus;
mod new_capl_scrits_nrdmp_blce;
mod otr_cpr_invstmnt_sttus;
mod outcmpny_drctr_nd_change_sttus;
mod pay_v2;
mod prvsrp_cptal_use_dtls;
mod pssrp_cptal_use_dtls;
mod srtpd_psndbt_nrdmp_blce;
mod stock_totqy_sttus;
mod tesstk_acqs_dsps_sttus;
mod unrst_exctv_mendng_sttus;

pub use accnut_adtor_nm_nd_adt_opinion::AuditOpinion;
pub use accnut_adtor_non_adt_servc_cncls_sttus::NonAuditContract;
pub use adt_servc_cncls_sttus::AuditContract;
pub use alot_matter::Dividend;
pub use cndl_capl_scrits_nrdmp_blce::ContingentCapitalBalance;
pub use cprnd_nrdmp_blce::CorporateBondBalance;
pub use det_scrits_isu_acmslt::DebtIssuance;
pub use drctr_adt_all_mendng_sttus_gmtsck_confm_amount::ApprovedDirectorPay;
pub use drctr_adt_all_mendng_sttus_mendng_pymntamt_ty_cl::DirectorPayByType;
pub use emp_sttus::Employees;
pub use entrprs_bil_scrits_nrdmp_blce::CommercialPaperBalance;
pub use exctv_sttus::Executive;
pub use hmv_audit_all_sttus::DirectorPayTotal;
pub use hmv_audit_indvdl_by_sttus::IndividualPay;
pub use hyslr_chg_sttus::LargestShareholderChange;
pub use hyslr_sttus::LargestShareholder;
pub use irds_sttus::CapitalChange;
pub use mrhl_sttus::MinorityShareholders;
pub use new_capl_scrits_nrdmp_blce::HybridSecuritiesBalance;
pub use otr_cpr_invstmnt_sttus::Investment;
pub use outcmpny_drctr_nd_change_sttus::OutsideDirectors;
pub use pay_v2::{PayRecipient, PayReport, StockBasedPay};
pub use prvsrp_cptal_use_dtls::PrivatePlacementFundUse;
pub use pssrp_cptal_use_dtls::PublicOfferingFundUse;
pub use srtpd_psndbt_nrdmp_blce::ShortTermBondBalance;
pub use stock_totqy_sttus::ShareCount;
pub use tesstk_acqs_dsps_sttus::TreasuryStock;
pub use unrst_exctv_mendng_sttus::UnregisteredExecutivePay;
