//! The row types of the registration statement groups.

use serde::Deserialize;

use crate::{CorpClass, Number, decode};

/// 일반사항 of an equity or depositary receipt registration: the subscription and
/// payment dates.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct OfferingGeneral {
    /// 접수번호: the 14-digit receipt number of the registration statement.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 청약기일
    #[serde(default)]
    pub sbd: String,
    /// 납입기일
    #[serde(default)]
    pub pymd: String,
    /// 청약공고일
    #[serde(default)]
    pub sband: String,
    /// 배정공고일
    #[serde(default)]
    pub asand: String,
    /// 배정기준일
    #[serde(default)]
    pub asstd: String,
    /// 신주인수권에 관한 사항(행사대상증권)
    #[serde(default)]
    pub exstk: String,
    /// 신주인수권에 관한 사항(행사가격)
    #[serde(default, deserialize_with = "decode::number")]
    pub exprc: Option<Number>,
    /// 신주인수권에 관한 사항(행사기간)
    #[serde(default)]
    pub expd: String,
    /// 주요사항보고서(접수번호)
    #[serde(default)]
    pub rpt_rcpn: String,
}

/// 일반사항 of a debt securities registration: the terms of one bond issue.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct BondGeneral {
    /// 접수번호: the 14-digit receipt number of the registration statement.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 회차: the bond's series number, in debt registrations only
    #[serde(default)]
    pub tm: String,
    /// 채무증권 명칭
    #[serde(default)]
    pub bdnmn: String,
    /// 모집(매출)방법
    #[serde(default)]
    pub slmth: String,
    /// 권면(전자등록)총액
    #[serde(default, deserialize_with = "decode::number")]
    pub fta: Option<Number>,
    /// 모집(매출)총액
    #[serde(default, deserialize_with = "decode::number")]
    pub slta: Option<Number>,
    /// 발행가액
    #[serde(default, deserialize_with = "decode::number")]
    pub isprc: Option<Number>,
    /// 이자율
    #[serde(default, deserialize_with = "decode::number")]
    pub intr: Option<Number>,
    /// 발행수익률
    #[serde(default, deserialize_with = "decode::number")]
    pub isrr: Option<Number>,
    /// 상환기일
    #[serde(default)]
    pub rpd: String,
    /// 원리금지급대행기관
    #[serde(default)]
    pub print_pymint: String,
    /// (사채)관리회사
    #[serde(default)]
    pub mngt_cmp: String,
    /// 신용등급(신용평가기관)
    #[serde(default)]
    pub cdrt_int: String,
    /// 청약기일
    #[serde(default)]
    pub sbd: String,
    /// 납입기일
    #[serde(default)]
    pub pymd: String,
    /// 청약공고일
    #[serde(default)]
    pub sband: String,
    /// 배정공고일
    #[serde(default)]
    pub asand: String,
    /// 배정기준일
    #[serde(default)]
    pub asstd: String,
    /// 표시통화
    #[serde(default)]
    pub dpcrn: String,
    /// 표시통화기준발행규모
    #[serde(default, deserialize_with = "decode::number")]
    pub dpcr_amt: Option<Number>,
    /// 사용지역
    #[serde(default)]
    pub usarn: String,
    /// 사용국가
    #[serde(default)]
    pub usntn: String,
    /// 원화 교환 예정 여부
    #[serde(default)]
    pub wnexpl_at: String,
    /// 인수기관명
    #[serde(default)]
    pub udtintnm: String,
    /// 보증을 받은 경우(보증기관)
    #[serde(default)]
    pub grt_int: String,
    /// 보증을 받은 경우(보증금액)
    #[serde(default, deserialize_with = "decode::number")]
    pub grt_amt: Option<Number>,
    /// 담보 제공의 경우(담보의 종류)
    #[serde(default)]
    pub icmg_mgknd: String,
    /// 담보 제공의 경우(담보금액)
    #[serde(default, deserialize_with = "decode::number")]
    pub icmg_mgamt: Option<Number>,
    /// 지분증권과 연계된 경우(행사대상증권)
    #[serde(default)]
    pub estk_exstk: String,
    /// 지분증권과 연계된 경우(권리행사비율)
    #[serde(default, deserialize_with = "decode::number")]
    pub estk_exrt: Option<Number>,
    /// 지분증권과 연계된 경우(권리행사가격)
    #[serde(default, deserialize_with = "decode::number")]
    pub estk_exprc: Option<Number>,
    /// 지분증권과 연계된 경우(권리행사기간)
    #[serde(default)]
    pub estk_expd: String,
    /// 주요사항보고서(접수번호)
    #[serde(default)]
    pub rpt_rcpn: String,
    /// 파생결합사채해당여부
    #[serde(default)]
    pub drcb_at: String,
    /// 파생결합사채(기초자산)
    #[serde(default)]
    pub drcb_uast: String,
    /// 파생결합사채(옵션종류)
    #[serde(default)]
    pub drcb_optknd: String,
    /// 파생결합사채(만기일)
    #[serde(default)]
    pub drcb_mtd: String,
}

/// 일반사항 of a merger, share exchange or split registration: the schedule and
/// terms.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct RestructuringGeneral {
    /// 접수번호: the 14-digit receipt number of the registration statement.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 형태
    #[serde(default)]
    pub stn: String,
    /// 이사회 결의일
    #[serde(default)]
    pub bddd: String,
    /// 계약일
    #[serde(default)]
    pub ctrd: String,
    /// 주주총회를 위한 주주확정일
    #[serde(default)]
    pub gmtsck_shddstd: String,
    /// 승인을 위한 주주총회일
    #[serde(default)]
    pub ap_gmtsck: String,
    /// 주식매수청구권 행사 기간 및 가격(시작일)
    #[serde(default)]
    pub aprskh_pd_bgd: String,
    /// 주식매수청구권 행사 기간 및 가격(종료일)
    #[serde(default)]
    pub aprskh_pd_edd: String,
    /// 주식매수청구권 행사 기간 및 가격((주식매수청구가격-회사제시))
    #[serde(default)]
    pub aprskh_prc: String,
    /// 합병기일등
    #[serde(default)]
    pub mgdt_etc: String,
    /// 비율 또는 가액
    #[serde(default)]
    pub rt_vl: String,
    /// 외부평가기관
    #[serde(default)]
    pub exevl_int: String,
    /// 지급 교부금 등
    #[serde(default)]
    pub grtmn_etc: String,
    /// 주요사항보고서(접수번호)
    #[serde(default)]
    pub rpt_rcpn: String,
}

/// 증권의종류: one class of securities offered.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct OfferedSecurity {
    /// 접수번호: the 14-digit receipt number of the registration statement.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 증권의종류
    #[serde(default)]
    pub stksen: String,
    /// 증권수량
    #[serde(default, deserialize_with = "decode::number")]
    pub stkcnt: Option<Number>,
    /// 액면가액
    #[serde(default, deserialize_with = "decode::number")]
    pub fv: Option<Number>,
    /// 모집(매출)가액
    #[serde(default, deserialize_with = "decode::number")]
    pub slprc: Option<Number>,
    /// 모집(매출)총액
    #[serde(default, deserialize_with = "decode::number")]
    pub slta: Option<Number>,
    /// 모집(매출)방법
    #[serde(default)]
    pub slmthn: String,
}

/// 인수인정보: one underwriter.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct Underwriter {
    /// 접수번호: the 14-digit receipt number of the registration statement.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 인수인구분
    #[serde(default)]
    pub actsen: String,
    /// 인수인명
    #[serde(default)]
    pub actnmn: String,
    /// 증권의종류
    #[serde(default)]
    pub stksen: String,
    /// 인수수량
    #[serde(default, deserialize_with = "decode::number")]
    pub udtcnt: Option<Number>,
    /// 인수금액
    #[serde(default, deserialize_with = "decode::number")]
    pub udtamt: Option<Number>,
    /// 인수대가
    #[serde(default)]
    pub udtprc: String,
    /// 인수방법
    #[serde(default)]
    pub udtmth: String,
    /// 회차: the bond's series number, in debt registrations only
    #[serde(default)]
    pub tm: String,
}

/// 자금의사용목적: one use of the funds raised.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct FundUse {
    /// 접수번호: the 14-digit receipt number of the registration statement.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 구분
    #[serde(default)]
    pub se: String,
    /// 금액
    #[serde(default, deserialize_with = "decode::number")]
    pub amt: Option<Number>,
    /// 회차: the bond's series number, in debt registrations only
    #[serde(default)]
    pub tm: String,
}

/// 매출인에관한사항: one holder selling securities in the offering.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct Seller {
    /// 접수번호: the 14-digit receipt number of the registration statement.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 보유자
    #[serde(default)]
    pub hdr: String,
    /// 회사와의관계
    #[serde(default)]
    pub rl_cmp: String,
    /// 매출전보유증권수
    #[serde(default, deserialize_with = "decode::number")]
    pub bfsl_hdstk: Option<Number>,
    /// 매출증권수
    #[serde(default, deserialize_with = "decode::number")]
    pub slstk: Option<Number>,
    /// 매출후보유증권수
    #[serde(default, deserialize_with = "decode::number")]
    pub atsl_hdstk: Option<Number>,
    /// 회차: the bond's series number, in debt registrations only
    #[serde(default)]
    pub tm: String,
}

/// 일반청약자환매청구권: the put-back right given to retail subscribers.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct PutBackRight {
    /// 접수번호: the 14-digit receipt number of the registration statement.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 부여사유
    #[serde(default)]
    pub grtrs: String,
    /// 행사가능 투자자
    #[serde(default)]
    pub exavivr: String,
    /// 부여수량
    #[serde(default, deserialize_with = "decode::number")]
    pub grtcnt: Option<Number>,
    /// 행사기간
    #[serde(default)]
    pub expd: String,
    /// 행사가격
    #[serde(default, deserialize_with = "decode::number")]
    pub exprc: Option<Number>,
}

/// 발행증권: one class of securities issued.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct IssuedSecurity {
    /// 접수번호: the 14-digit receipt number of the registration statement.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 종류
    #[serde(default)]
    pub kndn: String,
    /// 수량
    #[serde(default, deserialize_with = "decode::number")]
    pub cnt: Option<Number>,
    /// 액면가액
    #[serde(default, deserialize_with = "decode::number")]
    pub fv: Option<Number>,
    /// 모집(매출)가액
    #[serde(default, deserialize_with = "decode::number")]
    pub slprc: Option<Number>,
    /// 모집(매출)총액
    #[serde(default, deserialize_with = "decode::number")]
    pub slta: Option<Number>,
}

/// 당사회사에관한사항: one company taking part.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct PartyCompany {
    /// 접수번호: the 14-digit receipt number of the registration statement.
    pub rcept_no: String,
    /// 법인구분: the market.
    pub corp_cls: CorpClass,
    /// 고유번호: the company's 8-digit OpenDART code.
    pub corp_code: String,
    /// 회사명
    pub corp_name: String,
    /// 회사명
    #[serde(default)]
    pub cmpnm: String,
    /// 구분
    #[serde(default)]
    pub sen: String,
    /// 총자산
    #[serde(default, deserialize_with = "decode::number")]
    pub tast: Option<Number>,
    /// 자본금
    #[serde(default, deserialize_with = "decode::number")]
    pub cpt: Option<Number>,
    /// 발행주식수(주식의종류)
    #[serde(default)]
    pub isstk_knd: String,
    /// 발행주식수(주식수)
    #[serde(default, deserialize_with = "decode::number")]
    pub isstk_cnt: Option<Number>,
}
