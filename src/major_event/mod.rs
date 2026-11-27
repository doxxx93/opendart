//! 주요사항보고서 주요정보 (major event reports), OpenDART group `DS005`.
//!
//! Every endpoint takes a [`DateRangeRequest`](crate::DateRangeRequest):
//! one company and a range of receipt dates (data from 2015 on). Each
//! returns one row per report filed in the range, and an empty list when
//! there is none. Amounts, share counts and ratios are
//! [`Number`](crate::Number)s; ratios between companies in mergers, splits
//! and share exchanges (`1 : 0.1145836`, often followed by an explanation)
//! are text.
//!
//! ```no_run
//! use opendart::Client;
//! use opendart::DateRangeRequest;
//! use opendart::jiff::civil::date;
//!
//! # async fn run(dart: Client) -> Result<(), opendart::Error> {
//! let request = DateRangeRequest::new("00126380", date(2026, 1, 1), date(2026, 11, 27));
//! for decision in dart.tsstk_aq_decsn(&request).await? {
//!     println!("{} {:?}", decision.rcept_no, decision.aqpln_stk_ostk);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! | Endpoint | Method | Row | 개발가이드 |
//! |---|---|---|---|
//! | `astInhtrfEtcPtbkOpt.json` | [`Client::ast_inhtrf_etc_ptbk_opt`] | [`AssetTransferPutBack`] | [자산양수도(기타), 풋백옵션](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020018) |
//! | `dfOcr.json` | [`Client::df_ocr`] | [`DefaultEvent`] | [부도발생](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020019) |
//! | `bsnSp.json` | [`Client::bsn_sp`] | [`BusinessSuspension`] | [영업정지](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020020) |
//! | `ctrcvsBgrq.json` | [`Client::ctrcvs_bgrq`] | [`RehabilitationFiling`] | [회생절차 개시신청](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020021) |
//! | `dsRsOcr.json` | [`Client::ds_rs_ocr`] | [`DissolutionCause`] | [해산사유 발생](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020022) |
//! | `piicDecsn.json` | [`Client::piic_decsn`] | [`PaidInCapitalIncrease`] | [유상증자 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020023) |
//! | `fricDecsn.json` | [`Client::fric_decsn`] | [`BonusIssue`] | [무상증자 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020024) |
//! | `pifricDecsn.json` | [`Client::pifric_decsn`] | [`PaidInAndBonusIssue`] | [유무상증자 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020025) |
//! | `crDecsn.json` | [`Client::cr_decsn`] | [`CapitalReduction`] | [감자 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020026) |
//! | `bnkMngtPcbg.json` | [`Client::bnk_mngt_pcbg`] | [`CreditorManagementStart`] | [채권은행 등의 관리절차 개시](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020027) |
//! | `lwstLg.json` | [`Client::lwst_lg`] | [`Lawsuit`] | [소송 등의 제기](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020028) |
//! | `ovLstDecsn.json` | [`Client::ov_lst_decsn`] | [`OverseasListingDecision`] | [해외 증권시장 주권등 상장 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020029) |
//! | `ovDlstDecsn.json` | [`Client::ov_dlst_decsn`] | [`OverseasDelistingDecision`] | [해외 증권시장 주권등 상장폐지 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020030) |
//! | `ovLst.json` | [`Client::ov_lst`] | [`OverseasListing`] | [해외 증권시장 주권등 상장](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020031) |
//! | `ovDlst.json` | [`Client::ov_dlst`] | [`OverseasDelisting`] | [해외 증권시장 주권등 상장폐지](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020032) |
//! | `cvbdIsDecsn.json` | [`Client::cvbd_is_decsn`] | [`ConvertibleBondIssue`] | [전환사채권 발행결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020033) |
//! | `bdwtIsDecsn.json` | [`Client::bdwt_is_decsn`] | [`BondWithWarrantsIssue`] | [신주인수권부사채권 발행결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020034) |
//! | `exbdIsDecsn.json` | [`Client::exbd_is_decsn`] | [`ExchangeableBondIssue`] | [교환사채권 발행결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020035) |
//! | `bnkMngtPcsp.json` | [`Client::bnk_mngt_pcsp`] | [`CreditorManagementEnd`] | [채권은행 등의 관리절차 중단](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020036) |
//! | `wdCocobdIsDecsn.json` | [`Client::wd_cocobd_is_decsn`] | [`WriteDownContingentBondIssue`] | [상각형 조건부자본증권 발행결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020037) |
//! | `tsstkAqDecsn.json` | [`Client::tsstk_aq_decsn`] | [`TreasuryStockAcquisition`] | [자기주식 취득 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020038) |
//! | `tsstkDpDecsn.json` | [`Client::tsstk_dp_decsn`] | [`TreasuryStockDisposal`] | [자기주식 처분 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020039) |
//! | `tsstkAqTrctrCnsDecsn.json` | [`Client::tsstk_aq_trctr_cns_decsn`] | [`TreasuryTrustContract`] | [자기주식취득 신탁계약 체결 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020040) |
//! | `tsstkAqTrctrCcDecsn.json` | [`Client::tsstk_aq_trctr_cc_decsn`] | [`TreasuryTrustTermination`] | [자기주식취득 신탁계약 해지 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020041) |
//! | `bsnInhDecsn.json` | [`Client::bsn_inh_decsn`] | [`BusinessAcquisition`] | [영업양수 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020042) |
//! | `bsnTrfDecsn.json` | [`Client::bsn_trf_decsn`] | [`BusinessTransfer`] | [영업양도 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020043) |
//! | `tgastInhDecsn.json` | [`Client::tgast_inh_decsn`] | [`TangibleAssetAcquisition`] | [유형자산 양수 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020044) |
//! | `tgastTrfDecsn.json` | [`Client::tgast_trf_decsn`] | [`TangibleAssetTransfer`] | [유형자산 양도 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020045) |
//! | `otcprStkInvscrInhDecsn.json` | [`Client::otcpr_stk_invscr_inh_decsn`] | [`OtherCorpStockAcquisition`] | [타법인 주식 및 출자증권 양수결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020046) |
//! | `otcprStkInvscrTrfDecsn.json` | [`Client::otcpr_stk_invscr_trf_decsn`] | [`OtherCorpStockTransfer`] | [타법인 주식 및 출자증권 양도결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020047) |
//! | `stkrtbdInhDecsn.json` | [`Client::stkrtbd_inh_decsn`] | [`EquityLinkedBondAcquisition`] | [주권 관련 사채권 양수 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020048) |
//! | `stkrtbdTrfDecsn.json` | [`Client::stkrtbd_trf_decsn`] | [`EquityLinkedBondTransfer`] | [주권 관련 사채권 양도 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020049) |
//! | `cmpMgDecsn.json` | [`Client::cmp_mg_decsn`] | [`Merger`] | [회사합병 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020050) |
//! | `cmpDvDecsn.json` | [`Client::cmp_dv_decsn`] | [`CorporateSplit`] | [회사분할 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020051) |
//! | `cmpDvmgDecsn.json` | [`Client::cmp_dvmg_decsn`] | [`SplitMerger`] | [회사분할합병 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020052) |
//! | `stkExtrDecsn.json` | [`Client::stk_extr_decsn`] | [`ShareExchange`] | [주식교환·이전 결정](https://opendart.fss.or.kr/guide/detail.do?apiGrpCd=DS005&apiId=2020053) |
//!
//! [`Client::ast_inhtrf_etc_ptbk_opt`]: crate::Client::ast_inhtrf_etc_ptbk_opt
//! [`Client::df_ocr`]: crate::Client::df_ocr
//! [`Client::bsn_sp`]: crate::Client::bsn_sp
//! [`Client::ctrcvs_bgrq`]: crate::Client::ctrcvs_bgrq
//! [`Client::ds_rs_ocr`]: crate::Client::ds_rs_ocr
//! [`Client::piic_decsn`]: crate::Client::piic_decsn
//! [`Client::fric_decsn`]: crate::Client::fric_decsn
//! [`Client::pifric_decsn`]: crate::Client::pifric_decsn
//! [`Client::cr_decsn`]: crate::Client::cr_decsn
//! [`Client::bnk_mngt_pcbg`]: crate::Client::bnk_mngt_pcbg
//! [`Client::lwst_lg`]: crate::Client::lwst_lg
//! [`Client::ov_lst_decsn`]: crate::Client::ov_lst_decsn
//! [`Client::ov_dlst_decsn`]: crate::Client::ov_dlst_decsn
//! [`Client::ov_lst`]: crate::Client::ov_lst
//! [`Client::ov_dlst`]: crate::Client::ov_dlst
//! [`Client::cvbd_is_decsn`]: crate::Client::cvbd_is_decsn
//! [`Client::bdwt_is_decsn`]: crate::Client::bdwt_is_decsn
//! [`Client::exbd_is_decsn`]: crate::Client::exbd_is_decsn
//! [`Client::bnk_mngt_pcsp`]: crate::Client::bnk_mngt_pcsp
//! [`Client::wd_cocobd_is_decsn`]: crate::Client::wd_cocobd_is_decsn
//! [`Client::tsstk_aq_decsn`]: crate::Client::tsstk_aq_decsn
//! [`Client::tsstk_dp_decsn`]: crate::Client::tsstk_dp_decsn
//! [`Client::tsstk_aq_trctr_cns_decsn`]: crate::Client::tsstk_aq_trctr_cns_decsn
//! [`Client::tsstk_aq_trctr_cc_decsn`]: crate::Client::tsstk_aq_trctr_cc_decsn
//! [`Client::bsn_inh_decsn`]: crate::Client::bsn_inh_decsn
//! [`Client::bsn_trf_decsn`]: crate::Client::bsn_trf_decsn
//! [`Client::tgast_inh_decsn`]: crate::Client::tgast_inh_decsn
//! [`Client::tgast_trf_decsn`]: crate::Client::tgast_trf_decsn
//! [`Client::otcpr_stk_invscr_inh_decsn`]: crate::Client::otcpr_stk_invscr_inh_decsn
//! [`Client::otcpr_stk_invscr_trf_decsn`]: crate::Client::otcpr_stk_invscr_trf_decsn
//! [`Client::stkrtbd_inh_decsn`]: crate::Client::stkrtbd_inh_decsn
//! [`Client::stkrtbd_trf_decsn`]: crate::Client::stkrtbd_trf_decsn
//! [`Client::cmp_mg_decsn`]: crate::Client::cmp_mg_decsn
//! [`Client::cmp_dv_decsn`]: crate::Client::cmp_dv_decsn
//! [`Client::cmp_dvmg_decsn`]: crate::Client::cmp_dvmg_decsn
//! [`Client::stk_extr_decsn`]: crate::Client::stk_extr_decsn

mod ast_inhtrf_etc_ptbk_opt;
mod bdwt_is_decsn;
mod bnk_mngt_pcbg;
mod bnk_mngt_pcsp;
mod bsn_inh_decsn;
mod bsn_sp;
mod bsn_trf_decsn;
mod cmp_dv_decsn;
mod cmp_dvmg_decsn;
mod cmp_mg_decsn;
mod cr_decsn;
mod ctrcvs_bgrq;
mod cvbd_is_decsn;
mod df_ocr;
mod ds_rs_ocr;
mod exbd_is_decsn;
mod fric_decsn;
mod lwst_lg;
mod otcpr_stk_invscr_inh_decsn;
mod otcpr_stk_invscr_trf_decsn;
mod ov_dlst;
mod ov_dlst_decsn;
mod ov_lst;
mod ov_lst_decsn;
mod pifric_decsn;
mod piic_decsn;
mod stk_extr_decsn;
mod stkrtbd_inh_decsn;
mod stkrtbd_trf_decsn;
mod tgast_inh_decsn;
mod tgast_trf_decsn;
mod tsstk_aq_decsn;
mod tsstk_aq_trctr_cc_decsn;
mod tsstk_aq_trctr_cns_decsn;
mod tsstk_dp_decsn;
mod wd_cocobd_is_decsn;

pub use ast_inhtrf_etc_ptbk_opt::AssetTransferPutBack;
pub use bdwt_is_decsn::BondWithWarrantsIssue;
pub use bnk_mngt_pcbg::CreditorManagementStart;
pub use bnk_mngt_pcsp::CreditorManagementEnd;
pub use bsn_inh_decsn::BusinessAcquisition;
pub use bsn_sp::BusinessSuspension;
pub use bsn_trf_decsn::BusinessTransfer;
pub use cmp_dv_decsn::CorporateSplit;
pub use cmp_dvmg_decsn::SplitMerger;
pub use cmp_mg_decsn::Merger;
pub use cr_decsn::CapitalReduction;
pub use ctrcvs_bgrq::RehabilitationFiling;
pub use cvbd_is_decsn::ConvertibleBondIssue;
pub use df_ocr::DefaultEvent;
pub use ds_rs_ocr::DissolutionCause;
pub use exbd_is_decsn::ExchangeableBondIssue;
pub use fric_decsn::BonusIssue;
pub use lwst_lg::Lawsuit;
pub use otcpr_stk_invscr_inh_decsn::OtherCorpStockAcquisition;
pub use otcpr_stk_invscr_trf_decsn::OtherCorpStockTransfer;
pub use ov_dlst::OverseasDelisting;
pub use ov_dlst_decsn::OverseasDelistingDecision;
pub use ov_lst::OverseasListing;
pub use ov_lst_decsn::OverseasListingDecision;
pub use pifric_decsn::PaidInAndBonusIssue;
pub use piic_decsn::PaidInCapitalIncrease;
pub use stk_extr_decsn::ShareExchange;
pub use stkrtbd_inh_decsn::EquityLinkedBondAcquisition;
pub use stkrtbd_trf_decsn::EquityLinkedBondTransfer;
pub use tgast_inh_decsn::TangibleAssetAcquisition;
pub use tgast_trf_decsn::TangibleAssetTransfer;
pub use tsstk_aq_decsn::TreasuryStockAcquisition;
pub use tsstk_aq_trctr_cc_decsn::TreasuryTrustTermination;
pub use tsstk_aq_trctr_cns_decsn::TreasuryTrustContract;
pub use tsstk_dp_decsn::TreasuryStockDisposal;
pub use wd_cocobd_is_decsn::WriteDownContingentBondIssue;
