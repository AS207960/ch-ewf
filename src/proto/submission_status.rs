use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct GetSubmissionStatus {
    #[serde(rename = "$value")]
    pub reference: Option<GetSubmissionStatusReference>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}PresenterID")]
    pub presenter_id: String
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub enum GetSubmissionStatusReference {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CompanyNumber")]
    CompanyNumber(String),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}SubmissionNumber")]
    SubmissionNumber(String)
}

#[derive(Debug, Deserialize)]
pub struct SubmissionStatus {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Status",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Status",
    ), default)]
    pub status: Vec<Status>
}

#[derive(Debug, Deserialize)]
pub struct Status {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}SubmissionNumber",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}SubmissionNumber"
    ))]
    pub submission_number: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}StatusCode",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}StatusCode",
    ))]
    pub status_code: StatusCode,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}CompanyNumber",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}CompanyNumber",
    ), default)]
    pub company_number: Option<String>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}CustomerReference",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}CustomerReference",
    ), default
    )]
    pub customer_reference: Option<String>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Rejections",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Rejections",
    ), default)]
    pub rejections: Option<Rejections>,
    #[serde(rename = "$value", default)]
    pub details: Option<StatusDetails>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Examiner",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Examiner",
    ), default)]
    pub examiner: Option<Examiner>,
}

#[derive(Debug, Deserialize)]
pub enum StatusCode {
    #[serde(rename = "ACCEPT")]
    Accepted,
    #[serde(rename = "REJECT")]
    Rejected,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "PARKED")]
    Parked,
    #[serde(rename = "INTERNAL_FAILURE")]
    InternalFailure
}

#[derive(Debug, Deserialize)]
pub enum StatusDetails {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}IncorporationDetails",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}IncorporationDetails"
    ))]
    Incorporation(IncorporationDetails),
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}ChangeOfNameDetails",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}ChangeOfNameDetails"
    ))]
    ChangeOfName(ChangeOfNameDetails),
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}ChargeDetails",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}ChargeDetails"
    ))]
    Charge(ChargeDetails),
}

#[derive(Debug, Deserialize)]
pub struct Rejections {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Reject",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Reject",
    ), default)]
    pub rejections: Vec<Reject>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}RejectReference",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}RejectReference",
    ), default)]
    pub reject_reference: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Reject {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}RejectCode",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}RejectCode",
    ))]
    pub reject_code: i32,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Description",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Description",
    ))]
    pub description: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}InstanceNumber",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}InstanceNumber",
    ), default)]
    pub instance_number: Option<i32>
}

#[derive(Debug, Deserialize)]
pub struct IncorporationDetails {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}DocRequestKey",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}DocRequestKey"
    ))]
    pub document_request_key: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}IncorporationDate",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}IncorporationDate",
    ), deserialize_with = "super::deserialize_date")]
    pub incorporation_date: Date<Utc>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}AuthenticationCode",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}AuthenticationCode",
    ))]
    pub authentication_code: String
}

#[derive(Debug, Deserialize)]
pub struct ChangeOfNameDetails {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}DocRequestKey",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}DocRequestKey",
    ))]
    pub document_request_key: String,
}

#[derive(Debug, Deserialize)]
pub struct ChargeDetails {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}DocRequestKey",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}DocRequestKey",
    ))]
    pub document_request_key: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}ChargeCode",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}ChargeCode",
    ), default)]
    pub charge_code: String,
}

#[derive(Debug, Deserialize)]
pub struct Examiner {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Telephone",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Telephone",
    ))]
    pub telephone: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Comment",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Comment",
    ), default)]
    pub comment: Option<String>,
}