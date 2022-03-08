#[derive(Debug, Serialize, Clone)]
pub struct ChangeRegisteredOfficeAddress {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Address")]
    pub address: super::base_types::UKAddress
}