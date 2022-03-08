use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct ChargeSearch {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CompanyNumber")]
    pub company_number: u32,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CompanyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_type: Option<super::base_types::CompanyType>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CompanyAuthenticationCode")]
    pub company_authentication_code: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}StartDate",
        serialize_with = "super::serialize_date_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date: Option<Date<Utc>>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}EndDate",
        serialize_with = "super::serialize_date_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_date: Option<Date<Utc>>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ContinuationKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub continuation_key: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Charges {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}SearchRows")]
    pub search_rows: i32,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ContinuationKey", default)]
    pub continuation_key: Option<String>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Charges", default)]
    pub charges: Vec<Charge>
}

#[derive(Debug, Deserialize)]
pub struct Charge {
    #[serde(rename = "$value")]
    pub charge_id: ChargeID,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CreationDate",
        deserialize_with = "super::deserialize_date",
    )]
    pub creation_date: Date<Utc>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}AcquisitionDate",
        deserialize_with = "super::deserialize_date_opt",
        default
    )]
    pub acquisition_date: Option<Date<Utc>>,
    #[serde(rename = "$value")]
    pub description: ChargeDescription,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}PersonsEntitled")]
    pub persons_entitled: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}AdditionalPersonsEntitled", default)]
    pub additional_persons_entitled: bool,
}

#[derive(Debug, Deserialize)]
pub enum ChargeID {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ChargeCode")]
    ChargeCode(String),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ExistingChargeKey")]
    ExistingChargeKey(String)
}

#[derive(Debug, Deserialize)]
pub enum ChargeDescription {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ChargeDescription")]
    ChargeDescription(String),
    #[serde(rename = "$value")]
    InstrumentDescription(ChargeInstrumentDescription)
}

#[derive(Debug, Deserialize)]
pub struct ChargeInstrumentDescription {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}InstrumentDescription")]
    pub instrument_description: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ShortParticulars")]
    pub short_particulars: String,
}