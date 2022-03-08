#[derive(Debug, Serialize)]
pub struct GetERemindersRequest {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyNumber")]
    pub company_number: u32,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_type: Option<super::base_types::CompanyType>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyAuthenticationCode")]
    pub company_authentication_code: String,
}

#[derive(Debug, Serialize)]
pub struct SetERemindersRequest {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyNumber")]
    pub company_number: u32,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_type: Option<super::base_types::CompanyType>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyAuthenticationCode")]
    pub company_authentication_code: String,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}EmailAddress")]
    pub emails: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EReminders {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Recipient", default)]
    pub recipients: Vec<Recipient>
}

#[derive(Debug, Deserialize)]
pub struct Recipient {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}EmailAddress")]
    pub email: String,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Activated")]
    pub activated: bool,
}
