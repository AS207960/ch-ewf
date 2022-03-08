use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct PSCCessation {
    #[serde(rename = "$value")]
    pub entity: PSCEntity,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CessationDate",
        serialize_with = "super::serialize_date"
    )]
    pub cessation_date: Date<Utc>,
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
    Individual(super::psc::PSCIdentification),
}

#[derive(Debug, Serialize, Clone)]
pub struct Corporate {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CorporateName")]
    pub corporate_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct LegalPerson {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}LegalPersonName")]
    pub legal_person_name: String,
}