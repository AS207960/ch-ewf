use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct OfficerChangeDetails {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DateOfChange",
        serialize_with = "super::serialize_date"
    )]
    pub date_of_change: Date<Utc>,
    #[serde(rename = "$value")]
    pub change: ChangeType,
}

#[derive(Debug, Serialize, Clone)]
pub enum ChangeType {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Director")]
    Director(DirectorChange),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Secretary")]
    Secretary(Box<SecretaryChange>),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Member")]
    Member(MemberChange)
}

#[derive(Debug, Serialize, Clone)]
pub enum DirectorChange {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Person")]
    Person(Box<DirectorPersonChange>),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Corporate")]
    Corporate(Box<CorporateChangeType>)
}

#[derive(Debug, Serialize, Clone)]
pub enum SecretaryChange {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Person")]
    Person(SecretaryPersonChange),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Corporate")]
    Corporate(CorporateChangeType)
}

#[derive(Debug, Serialize, Clone)]
pub enum MemberChange {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Person")]
    Person(Box<MemberPersonChange>),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Corporate")]
    Corporate(Box<CorporateMemberChangeType>)
}

#[derive(Debug, Serialize, Clone)]
pub struct DirectorPersonChange {
    #[serde(rename = "$value")]
    pub person: super::base_types::PersonType,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DOB",
        serialize_with = "super::serialize_date"
    )]
    pub dob: Date<Utc>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Change",
        skip_serializing_if = "Option::is_none"
    )]
    pub change: Option<DirectorPersonChangeDetails>
}

#[derive(Debug, Serialize, Clone)]
pub struct DirectorPersonChangeDetails {
    #[serde(rename = "$value")]
    pub person_change: PersonChangeDetails,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ResidentialAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub residential_address: Option<super::base_types::ResidentialAddressType>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Nationality",
        skip_serializing_if = "Option::is_none"
    )]
    pub nationality: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CountryOfResidence",
        skip_serializing_if = "Option::is_none"
    )]
    pub country_of_residence: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Occupation",
        skip_serializing_if = "Option::is_none"
    )]
    pub occupation: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SecretaryPersonChange {
    #[serde(rename = "$value")]
    pub person: super::base_types::PersonType,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Change",
        skip_serializing_if = "Option::is_none"
    )]
    pub change: Option<PersonChangeDetails>
}

#[derive(Debug, Serialize, Clone)]
pub struct MemberPersonChange {
    #[serde(rename = "$value")]
    pub person: super::base_types::PersonType,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DOB",
        serialize_with = "super::serialize_date"
    )]
    pub dob: Date<Utc>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Change",
        skip_serializing_if = "Option::is_none"
    )]
    pub change: Option<MemberPersonChangeDetails>
}

#[derive(Debug, Serialize, Clone)]
pub struct MemberPersonChangeDetails {
    #[serde(rename = "$value")]
    pub person_change: PersonChangeDetails,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ResidentialAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub residential_address: Option<super::base_types::ResidentialAddressType>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CountryOfResidence",
        skip_serializing_if = "Option::is_none"
    )]
    pub country_of_residence: Option<String>,
    #[serde(
        rename = "$value",
        skip_serializing_if = "Option::is_none"
    )]
    pub designated: Option<MemberDesignated>,
}

#[derive(Debug, Serialize, Clone)]
pub struct MemberDesignated {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}DesignatedInd")]
    pub designated: bool,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ConsentToAct")]
    pub consent_to_act: bool
}

#[derive(Debug, Serialize, Clone)]
pub struct PersonChangeDetails {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Name",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<super::base_types::PersonType>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ServiceAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_address: Option<ServiceAddressChange>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ServiceAddressChange {
    #[serde(rename = "$value")]
    pub address: super::base_types::ServiceAddressType,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ResidentialAddressUnchangedInd",
        skip_serializing_if = "super::is_false"
    )]
    pub residential_address_unchanged: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct CorporateChangeType {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CorporateName")]
    pub corporate_name: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Change",
        skip_serializing_if = "Option::is_none"
    )]
    pub change: Option<CorporateChangeDetails>
}

#[derive(Debug, Serialize, Clone)]
pub struct CorporateChangeDetails {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CorporateName",
        skip_serializing_if = "Option::is_none"
    )]
    pub corporate_name: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Address",
        skip_serializing_if = "Option::is_none"
    )]
    pub address: Option<super::base_types::CompanyAddress>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CompanyIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_identification: Option<super::base_types::CompanyIdentification>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CorporateMemberChangeType {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CorporateName")]
    pub corporate_name: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Change",
        skip_serializing_if = "Option::is_none"
    )]
    pub change: Option<CorporateMemberChangeDetails>
}

#[derive(Debug, Serialize, Clone)]
pub struct CorporateMemberChangeDetails {
    #[serde(rename = "$value")]
    pub change: CorporateChangeDetails,
    #[serde(
        rename = "$value",
        skip_serializing_if = "Option::is_none"
    )]
    pub designated: Option<MemberDesignated>,
}