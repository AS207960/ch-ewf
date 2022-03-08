use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct ReturnOfAllotmentShares {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}StartPeriodSharesAllotted",
        serialize_with = "super::serialize_date"
    )]
    pub start_period: Date<Utc>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}EndPeriodSharesAllotted",
        serialize_with = "super::serialize_date_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_period: Option<Date<Utc>>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}StatementOfCapital")]
    pub statement_of_capital: super::base_types::StatementOfCapital,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Allotment")]
    pub allotment: Vec<Allotment>
}

#[derive(Debug, Serialize, Clone)]
pub struct Allotment {
    #[serde(rename = "$value")]
    pub allotment: super::base_types::Allotment,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Consideration",
        skip_serializing_if = "Option::is_none"
    )]
    pub consideration: Option<String>
}