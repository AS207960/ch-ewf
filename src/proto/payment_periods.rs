use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct PaymentPeriodsRequest {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CompanyNumber")]
    pub company_number: u32,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CompanyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_type: Option<super::base_types::CompanyType>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CompanyAuthenticationCode")]
    pub company_authentication_code: String,
}

#[derive(Debug, Deserialize)]
pub struct PaymentPeriods {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/v1-0/schema}PaymentPeriod")]
    pub periods: Vec<PaymentPeriod>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentPeriod {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/v1-0/schema}StartDate",
        deserialize_with = "super::deserialize_date"
    )]
    pub start_date: Date<Utc>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/v1-0/schema}EndDate",
        deserialize_with = "super::deserialize_date"
    )]
    pub end_date: Date<Utc>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/v1-0/schema}PeriodPaid")]
    pub paid: bool,
}
