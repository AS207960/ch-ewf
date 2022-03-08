use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct OfficerResignation {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ResignationDate",
        serialize_with = "super::serialize_date"
    )]
    pub resignation_date: Date<Utc>,
    #[serde(rename = "$value")]
    pub resignation: ResignationType,
}

#[derive(Debug, Serialize, Clone)]
pub enum ResignationType {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Director")]
    Director(DirectorResignation),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Secretary")]
    Secretary(SecretaryResignation),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Member")]
    Member(MemberResignation)
}

#[derive(Debug, Serialize, Clone)]
pub enum DirectorResignation {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CorporateName")]
    CorporateName(String),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Person")]
    Person(PersonChange)
}

#[derive(Debug, Serialize, Clone)]
pub enum SecretaryResignation {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CorporateName")]
    CorporateName(String),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Person")]
    Person(super::base_types::PersonType)
}

#[derive(Debug, Serialize, Clone)]
pub enum MemberResignation {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CorporateName")]
    CorporateName(String),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Person")]
    Person(PersonChange)
}

#[derive(Debug, Serialize, Clone)]
pub struct PersonChange {
    #[serde(rename = "$value")]
    pub person: super::base_types::PersonType,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DOB",
        serialize_with = "super::serialize_date"
    )]
    pub dob: Date<Utc>,
}