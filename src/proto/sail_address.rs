#[derive(Debug, Serialize, Clone)]
pub struct SAILAddress {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Address")]
    pub address: super::base_types::UKAddress
}