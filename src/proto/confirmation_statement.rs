use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct ConfirmationStatement {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}TradingOnMarket",
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_on_market: Option<bool>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DTR5Applies",
        skip_serializing_if = "Option::is_none"
    )]
    pub dtr5_applies: Option<bool>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}PSCExemptAsTradingOnRegulatedMarket",
        skip_serializing_if = "Option::is_none"
    )]
    pub psc_exempt_as_trading_on_regulated_market: Option<bool>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}PSCExemptAsSharesAdmittedOnMarket",
        skip_serializing_if = "Option::is_none"
    )]
    pub psc_exempt_as_shares_admitted_on_market: Option<bool>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}PSCExemptAsTradingOnUKRegulatedMarket",
        skip_serializing_if = "Option::is_none"
    )]
    pub psc_exempt_as_trading_on_uk_regulated_market: Option<bool>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ReviewDate",
        serialize_with = "super::serialize_date"
    )]
    pub review_date: Date<Utc>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}SICCodes",
        skip_serializing_if = "Option::is_none"
    )]
    pub sic_codes: Option<super::base_types::SICCodes>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}StatementOfCapital",
        skip_serializing_if = "Option::is_none"
    )]
    pub statement_of_capital: Option<super::base_types::StatementOfCapital>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Shareholdings",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub shareholdings: Vec<Shareholding>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}StateConfirmation")]
    pub state_confirmation: bool
}

#[derive(Debug, Serialize, Clone)]
pub struct Shareholding {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ShareClass")]
    pub share_class: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}NumberHeld")]
    pub number_held: f64,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Transfers",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transfers: Vec<Transfer>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Shareholders",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub shareholders: Vec<Shareholder>
}

#[derive(Debug, Serialize, Clone)]
pub struct Transfer {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DateOfTransfer",
        serialize_with = "super::serialize_date"
    )]
    pub date_of_transfer: Date<Utc>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}NumberSharesTransferred")]
    pub number_of_shares_transferred: f64,
}

#[derive(Debug, Serialize, Clone)]
pub struct Shareholder {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Name")]
    pub name: ShareholderName,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Address")]
    pub address: super::base_types::BaseAddress
}

#[derive(Debug, Serialize, Clone)]
pub enum ShareholderName {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}AmalgamatedName")]
    AmalgamatedName(String),
    #[serde(rename = "$value")]
    Name(Name)
}

#[derive(Debug, Serialize, Clone)]
pub struct Name {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Surname")]
    pub surname: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Forename", default)]
    pub forename: Option<String>,
}