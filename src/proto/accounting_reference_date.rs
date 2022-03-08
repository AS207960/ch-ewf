use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct ChangeAccountingReferenceDate {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}AccountRefDate",
        serialize_with = "super::serialize_date"
    )]
    pub accounting_reference_date: Date<Utc>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ChangeToPeriod")]
    pub change_to_period: ChangeToPeriod,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}AmendedAccountRefDate",
        serialize_with = "super::serialize_date"
    )]
    pub amended_accounting_reference_date: Date<Utc>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}FiveYearExtensionDetails",
        skip_serializing_if = "Option::is_none"
    )]
    pub five_year_extension_details: Option<FiveYearExtensionDetails>,
}

#[derive(Debug, Serialize, Clone)]
pub enum ChangeToPeriod {
    #[serde(rename = "SHORT")]
    Shorten,
    #[serde(rename = "EXTEND")]
    Extend,
}

#[derive(Debug, Serialize, Clone)]
pub struct FiveYearExtensionDetails {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ExtensionReason")]
    pub extension_reason: ExtensionReason,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ExtensionAuthorisedCode")]
    pub extension_authorised_code: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub enum ExtensionReason {
    #[serde(rename = "ADMIN")]
    Administration,
    #[serde(rename = "STATE")]
    SecretaryOfState,
    #[serde(rename = "UKPAR")]
    UKParent
}