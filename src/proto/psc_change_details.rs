use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct PSCChangeDetails {
    #[serde(rename = "$value")]
    pub entity: PSCEntity,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DateOfChange",
        serialize_with = "super::serialize_date"
    )]
    pub date_of_change: Date<Utc>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}RegisterEntryDate",
        serialize_with = "super::serialize_date"
    )]
    pub register_entry_date: Date<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub enum PSCEntity {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Corporate")]
    Corporate(Corporate),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}LegalPerson")]
    LegalPerson(LegalPerson),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Individual")]
    Individual(Box<Individual>),
}

#[derive(Debug, Serialize, Clone)]
pub struct Individual {
    #[serde(rename = "$value")]
    pub identification: super::psc::PSCIdentification,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Change",
        skip_serializing_if = "Option::is_none"
    )]
    pub change: Option<IndividualChange>
}

#[derive(Debug, Serialize, Clone)]
pub struct IndividualChange {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Name",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<super::base_types::PersonType>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ServiceAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_address: Option<super::base_types::ServiceAddressType>,
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
        rename = "$value",
        skip_serializing_if = "Option::is_none"
    )]
    pub nature_of_controls: Option<super::psc::PSCNatureOfControls>
}

#[derive(Debug, Serialize, Clone)]
pub struct Corporate {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CorporateName")]
    pub corporate_name: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Change",
        skip_serializing_if = "Option::is_none"
    )]
    pub change: Option<CorporateChange>
}

#[derive(Debug, Serialize, Clone)]
pub struct CorporateChange {
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
        rename = "{http://xmlgw.companieshouse.gov.uk}PSCCompanyIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_identification: Option<super::psc::PSCCorporateIdentification>,
    #[serde(
        rename = "$value",
        skip_serializing_if = "Option::is_none"
    )]
    pub nature_of_controls: Option<super::psc::PSCNatureOfControls>
}

#[derive(Debug, Serialize, Clone)]
pub struct LegalPerson {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}LegalPersonName")]
    pub legal_person_name: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Change",
        skip_serializing_if = "Option::is_none"
    )]
    pub change: Option<LegalPersonChange>
}

#[derive(Debug, Serialize, Clone)]
pub struct LegalPersonChange {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}LegalPersonName",
        skip_serializing_if = "Option::is_none"
    )]
    pub legal_person_name: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Address",
        skip_serializing_if = "Option::is_none"
    )]
    pub address: Option<super::base_types::CompanyAddress>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}LegalPersonIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub legal_person_identification: Option<super::psc::PSCLegalPersonIdentification>,
    #[serde(
        rename = "$value",
        skip_serializing_if = "Option::is_none"
    )]
    pub nature_of_controls: Option<super::psc::PSCNatureOfControls>
}