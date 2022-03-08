use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct ChargeUpdate {
    #[serde(rename = "$value")]
    pub charge_id: ChargeID,
    #[serde(rename = "$value")]
    pub update: Update,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}PersonDelivering")]
    pub person_delivering: PersonDelivering,
}

#[derive(Debug, Serialize, Clone)]
pub enum ChargeID {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ChargeCode")]
    ChargeCode(String),
    #[serde(rename = "$value")]
    ExistingChargeKey(ExistingChargeKey)
}

#[derive(Debug, Serialize, Clone)]
pub struct ExistingChargeKey {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ExistingChargeKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub existing_charge_key: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CreationDate",
        serialize_with = "super::serialize_date"
    )]
    pub creation_date: Date<Utc>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}InstrumentDescription")]
    pub instrument_description: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ShortParticulars")]
    pub short_particulars: String,
}

#[derive(Debug, Serialize, Clone)]
pub enum Update {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Satisfaction")]
    Satisfaction(Satisfaction),
    #[serde(rename = "$value")]
    PartCeaseOrRelease(PartCeaseOrRelease),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}AllCeaseRelease")]
    AllCeaseOrRelease(CeaseOrReleaseType),
}

#[derive(Debug, Serialize, Clone)]
pub struct PersonDelivering {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Name")]
    pub name: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Address")]
    pub address: super::base_types::CompanyAddress,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}InterestInCharge")]
    pub interest_in_charge: String
}

#[derive(Debug, Serialize, Clone)]
pub enum Satisfaction {
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "PART")]
    Part
}

#[derive(Debug, Serialize, Clone)]
pub struct PartCeaseOrRelease {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}PartCeaseRelease")]
    pub cease_or_release: CeaseOrReleaseType,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}AssetsDescription")]
    pub assets_description: String,
}

#[derive(Debug, Serialize, Clone)]
pub enum CeaseOrReleaseType {
    #[serde(rename = "CEASE")]
    Cease,
    #[serde(rename = "RELEASE")]
    Release,
    #[serde(rename = "CEASEANDRELEASE")]
    CeaseAndRelease
}
