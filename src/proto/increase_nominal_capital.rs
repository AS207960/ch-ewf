#![allow(dead_code)]

use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct IncreaseNominalCapital {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ResolutionDate",
        serialize_with = "super::serialize_date"
    )]
    pub resolution_date: Date<Utc>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Currency")]
    pub currency: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}AmountOfIncrease")]
    pub amount_of_increase: f64,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Conditions")]
    pub conditions: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}NominalCapital")]
    pub nominal_capital: f64,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Resolution")]
    pub resolution: Resolution
}

#[derive(Debug, Serialize, Clone)]
pub struct Resolution {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Capital")]
    pub capital: Vec<super::base_types::Capital>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ResolutionType")]
    pub resolution_type: ResolutionType,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}MeetingType")]
    pub meeting_type: MeetingType,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Address",
        skip_serializing_if = "Option::is_none"
    )]
    pub address: Option<super::base_types::BaseAddress>
}

#[derive(Debug, Serialize, Clone)]
pub enum ResolutionType {
    #[serde(rename = "ORDINARY")]
    Ordinary,
    #[serde(rename = "SPECIAL")]
    Special,
    #[serde(rename = "EXTRAORDINARY")]
    Extraordinary,
}

#[derive(Debug, Serialize, Clone)]
pub enum MeetingType {
    #[serde(rename = "ANNUAL")]
    Annual,
    #[serde(rename = "EXTRAORDINARY")]
    Extraordinary
}