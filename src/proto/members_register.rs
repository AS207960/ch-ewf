use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub enum MembersRegisterElectOrWithdraw {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ElectToHold")]
    ElectToHold(ElectToHold),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}WithdrawElectionToHold")]
    WithdrawElectionToHold(bool)
}

#[derive(Debug, Serialize, Clone)]
pub struct ElectToHold {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Members")]
    pub members: Members,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}StateAllMembersAssented")]
    pub state_all_members_assented: bool,
    #[serde(
    rename = "{http://xmlgw.companieshouse.gov.uk}StateOverseasRegistersDiscontinued",
        skip_serializing_if = "super::is_false"
    )]
    pub state_overseas_registers_discontinued: bool,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}StateSingleMemberCompany",
        skip_serializing_if = "super::is_false"
    )]
    pub state_single_member_company: bool,
}

#[derive(Debug, Serialize, Clone)]
pub enum Members {
    #[serde(rename = "$value")]
    MembersWithShares(Vec<MemberWithShares>),
    #[serde(rename = "$value")]
    MembersWithoutShares(Vec<MemberWithoutShares>)
}

#[derive(Debug, Serialize, Clone)]
pub struct MemberWithShares {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}SharesOrStockHeld")]
    pub shares_or_stock_held: Vec<super::base_types::StocksOrSharesHeld>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Name")]
    pub name: Vec<super::base_types::CompanyMemberName>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Address")]
    pub address: super::base_types::BaseAddress,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DateRegisteredAsMember",
        serialize_with = "super::serialize_date"
    )]
    pub date_registered_as_member: Date<Utc>
}

#[derive(Debug, Serialize, Clone)]
pub struct MemberWithoutShares {
    #[serde(rename = "$value")]
    pub member: super::base_types::CompanyMember,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DateRegisteredAsMember",
        serialize_with = "super::serialize_date"
    )]
    pub date_registered_as_member: Date<Utc>
}