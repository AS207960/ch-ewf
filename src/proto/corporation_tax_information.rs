use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct CorporationTaxInformation {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}AbbreviatedCompanyName",
        skip_serializing_if = "Option::is_none"
    )]
    pub abbreviated_company_name: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}FirstAccountingPeriodStartDate",
        serialize_with = "super::serialize_date"
    )]
    pub first_accounting_period_start_date: Date<Utc>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}AccountsMadeUpDate",
        serialize_with = "super::serialize_date"
    )]
    pub accounts_made_up_date: Date<Utc>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}CT61MayApply")]
    pub ct61_may_apply: bool,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}PrincipalPlaceOfBusiness")]
    pub principal_place_of_business: PrincipalPlaceOfBusiness,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}CompanyHasTakenOverABusiness",
        skip_serializing_if = "Option::is_none"
    )]
    pub taken_over_business: Option<TakenOverBusiness>,
}

#[derive(Debug, Serialize, Clone)]
pub enum PrincipalPlaceOfBusiness {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}SameAsRegisteredOfficeAddress")]
    SameAsRegisteredOffice(bool),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}Address")]
    Address(Address)
}

#[derive(Debug, Serialize, Clone)]
pub struct Address {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}AddressLine1")]
    pub address_line_1: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}AddressLine2")]
    pub address_line_2: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}AddressLine3",
        skip_serializing_if = "Option::is_none"
    )]
    pub address_line_3: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}AddressLine4",
        skip_serializing_if = "Option::is_none"
    )]
    pub address_line_4: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}Postcode",
        skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}Country",
        skip_serializing_if = "Option::is_none"
    )]
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TakenOverBusiness {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}AddressLine2")]
    pub previous_business: PreviousBusiness,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}AddressLine2")]
    pub previous_owner: PreviousOwner,
}

#[derive(Debug, Serialize, Clone)]
pub struct PreviousBusiness {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}BusinessName")]
    pub business_name: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}BusinessType",
        skip_serializing_if = "Option::is_none"
    )]
    pub business_type: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}CompanyRegistrationNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_registration_number: Option<String>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}Address")]
    pub address: Address
}

#[derive(Debug, Serialize, Clone)]
pub struct PreviousOwner {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}OwnersName")]
    pub owner_name: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/HMRC}Address")]
    pub address: Address
}