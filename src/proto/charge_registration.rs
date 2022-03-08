use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct ChargeRegistration {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CreationDate",
        serialize_with = "super::serialize_date",
    )]
    pub creation_date: Date<Utc>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}PropertyAcquiredDate",
        serialize_with = "super::serialize_date_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub property_acquired_date: Option<Date<Utc>>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}PersonsEntitled")]
    pub persons_entitled: PersonsEntitled,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ChargeDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub charge_description: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}FixedChargeOrFixedSecurity",
        skip_serializing_if = "super::is_false"
    )]
    pub fixed_charge: bool,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}FloatingCharge",
        skip_serializing_if = "Option::is_none"
    )]
    pub floating_charge: Option<FloatingCharge>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}NegativePledge",
        skip_serializing_if = "super::is_false"
    )]
    pub negative_pledge: bool,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}BareTrustee",
        skip_serializing_if = "super::is_false"
    )]
    pub bare_trustee: bool,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}DeedCertificationStatement")]
    pub deed_certification_statement: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}DeedCertifiedBy")]
    pub deed_certified_by: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Authentication")]
    pub authentication: Vec<super::base_types::PersonalAttribute>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PersonsEntitled {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ChargeeName")]
    pub chargee_names: Vec<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}AdditionalChargees",
        skip_serializing_if = "super::is_false"
    )]
    pub additional_chargees: bool,
}

#[derive(Debug, Serialize, Clone)]
pub enum FloatingCharge {
    #[serde(rename = "N/A")]
    NA,
    #[serde(rename = "COVERSALL")]
    CoversAll,
    #[serde(rename = "DOESNOTCOVERALL")]
    DoesNotCoverAll
}
