use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct MembersDataRequest {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyNumber")]
    pub company_number: u32,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_type: Option<super::base_types::CompanyType>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyAuthenticationCode")]
    pub company_authentication_code: String,
}

#[derive(Debug, Deserialize)]
pub struct MembersData {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Members")]
    pub members: MembersDataMembers,
}

#[derive(Debug, Deserialize)]
pub struct MembersDataMembers {
    #[serde(rename = "$value")]
    pub members: Vec<MemberType>
}

#[derive(Debug, Deserialize)]
pub enum MemberType {
    MemberWithShares(MemberWithShares),
    Member(Member),
}

#[derive(Debug, Deserialize)]
pub struct MemberWithShares {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}StocksOrSharesHeld")]
    pub stocks_or_shares: Vec<super::register::StocksOrSharesHeld>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Name")]
    pub name: Vec<super::register::MemberName>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Address")]
    pub address: super::base_types::BaseAddress,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}DateRegisteredAsMember",
        deserialize_with = "super::deserialize_date"
    )]
    pub date_registered: Date<Utc>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}DateCeasedToBeMember",
        deserialize_with = "super::deserialize_date_opt",
        default
    )]
    pub date_ceased: Option<Date<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct Member {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}MemberClass")]
    pub class: String,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Name")]
    pub name: super::register::MemberName,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Address")]
    pub address: super::base_types::BaseAddress,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}DateRegisteredAsMember",
        deserialize_with = "super::deserialize_date"
    )]
    pub date_registered: Date<Utc>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}DateCeasedToBeMember",
        deserialize_with = "super::deserialize_date_opt",
        default
    )]
    pub date_ceased: Option<Date<Utc>>,
}
