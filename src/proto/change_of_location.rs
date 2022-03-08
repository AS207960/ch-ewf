#[derive(Debug, Serialize, Clone)]
pub enum RecordChangeOfLocation {
    #[serde(rename = "$value")]
    MoveToSAILAddress(MoveToSAILAddress),
    #[serde(rename = "$value")]
    MoveToRegisteredOffice(MoveToRegisteredOffice)
}

#[derive(Debug, Serialize, Clone)]
pub struct MoveToSAILAddress {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}MoveToSAILAddress")]
    pub move_to_sail_address: bool,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}RegisterList")]
    pub register_list: Vec<Register>
}

#[derive(Debug, Serialize, Clone)]
pub struct MoveToRegisteredOffice {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}MoveToRegisteredOffice")]
    pub move_to_registered_office: bool,
    #[serde(rename = "$value")]
    pub move_type: MoveToRegisteredOfficeType,
}

#[derive(Debug, Serialize, Clone)]
pub enum MoveToRegisteredOfficeType {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}RemoveSAILAddressInd")]
    All(bool),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}RegisterList")]
    Some(Vec<Register>)
}

#[derive(Debug, Serialize, Clone)]
pub struct Register {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}RecordType")]
    pub register_type: super::base_types::RecordType
}