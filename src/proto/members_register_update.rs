use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct MembersRegisterUpdate {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Members")]
    pub members: Members,
    #[serde(
        rename = "$value",
        skip_serializing_if = "Option::is_none"
    )]
    pub single_member: Option<SingleMember>,
}

#[derive(Debug, Serialize, Clone)]
pub enum SingleMember {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}StateNoLongerSingleMember")]
    StateNoLongerSingleMember(bool),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}StateBecomingSingleMember")]
    StateBecomingSingleMember(bool)
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
    #[serde(rename = "$value")]
    pub new_existing_or_ceased_member: NewExistingOrCeasedMemberWithShares,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Transfers")]
    pub transfers: Vec<Transfer>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Allotments")]
    pub allotments: Vec<Allotment>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Name")]
    pub name: Vec<super::base_types::CompanyMemberName>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Address")]
    pub address: super::base_types::BaseAddress,
}

#[derive(Debug, Serialize, Clone)]
pub struct Transfer {
    #[serde(rename = "$value")]
    pub share: super::base_types::Shares,
    #[serde(
    rename = "{http://xmlgw.companieshouse.gov.uk}TransferDate",
        serialize_with = "super::serialize_date"
    )]
    pub transfer_date: Date<Utc>
}

#[derive(Debug, Serialize, Clone)]
pub struct Allotment {
    #[serde(rename = "$value")]
    pub share: super::base_types::Shares,
    #[serde(
    rename = "{http://xmlgw.companieshouse.gov.uk}AllotmentDate",
        serialize_with = "super::serialize_date"
    )]
    pub allotment_date: Date<Utc>
}

#[derive(Debug, Serialize, Clone)]
pub struct MemberWithoutShares {
    #[serde(rename = "$value")]
    pub member: super::base_types::CompanyMember,
    #[serde(rename = "$value")]
    pub new_existing_or_ceased_member: NewExistingOrCeasedMemberWithoutShares
}

#[derive(Debug, Serialize, Clone)]
pub enum NewExistingOrCeasedMemberWithoutShares {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}NewOrExistingMember")]
    NewOrExistingMember(NewOrExistingMember),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CeasedToBeMember")]
    CeasedToBeMember(CeasedToBeMember)
}

#[derive(Debug, Serialize, Clone)]
pub enum NewExistingOrCeasedMemberWithShares {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}NewOrExistingMember")]
    NewOrExistingMember(NewOrExistingMemberWithShares),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CeasedToBeMember")]
    CeasedToBeMember(CeasedToBeMember)
}

#[derive(Debug, Serialize, Clone)]
pub struct NewOrExistingMemberWithShares {
    #[serde(rename = "$value")]
    pub new_or_existing_member: NewOrExistingMember,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}SharesOrStockHeld")]
    pub shares_or_stock_held: Vec<super::base_types::StocksOrSharesHeld>
}

#[derive(Debug, Serialize, Clone)]
pub enum NewOrExistingMember {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}StateNewMember")]
    StateNewMember(bool),
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DateRegisteredAsMember",
        serialize_with = "super::serialize_date"
    )]
    DateRegisteredAsMember(Date<Utc>)
}

#[derive(Debug, Serialize, Clone)]
pub struct CeasedToBeMember {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DateRegisteredAsMember",
        serialize_with = "super::serialize_date"
    )]
    pub date_registered_as_member: Date<Utc>,
    #[serde(rename = "$value")]
    pub ceasation_type: CeasedToBeMemberType
}

#[derive(Debug, Serialize, Clone)]
pub enum CeasedToBeMemberType {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}StateCeasedToBeMember")]
    StateCeasedToBeMember(bool),
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DateCeasedToBeMember",
        serialize_with = "super::serialize_date"
    )]
    DateCeasedToBeMember(Date<Utc>)
}