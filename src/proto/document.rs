use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct GetDocument {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}DocRequestKey")]
    pub document_request_key: String
}

#[derive(Debug, Deserialize)]
pub struct Document {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}CompanyNumber",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}CompanyNumber"
    ))]
    pub company_number: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}DocumentDate",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}DocumentDate",
    ), default, deserialize_with = "super::deserialize_date_opt")]
    pub document_date: Option<Date<Utc>>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}DocumentType",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}DocumentType"
    ), default)]
    pub document_type: Option<String>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}DocumentID",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}DocumentID"
    ))]
    pub document_id: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}DocumentData",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}DocumentData"
    ))]
    pub document_data: DocumentData,
}

#[derive(Debug, Deserialize)]
pub struct DocumentData {
    #[serde(rename = "$attr:content-type")]
    pub content_type: ContentType,
    #[serde(rename = "$attr:content-encoding")]
    pub content_encoding: ContentEncoding,
    #[serde(rename = "$attr:filename", default)]
    pub filename: Option<String>,
    #[serde(rename = "$value")]
    pub contents: String
}

#[derive(Debug, Deserialize)]
pub enum ContentType {
    #[serde(rename = "application.pdf")]
    Pdf
}

#[derive(Debug, Deserialize)]
pub enum ContentEncoding {
    #[serde(rename = "base64")]
    Base64
}