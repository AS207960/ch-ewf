use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkRoot {
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope;http://xmlgw.companieshouse.gov.uk/v2-1/schema/Egov_ch-v2-0.xsd}GovTalkMessage",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}GovTalkMessage"
    )]
    pub message: GovTalkMessage
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkMessage {
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}EnvelopeVersion",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}EnvelopeVersion"
    )]
    pub envelope_version: String,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Header",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Header"
    )]
    pub header: GovTalkHeader,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}GovTalkDetails",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}GovTalkDetails"
    )]
    pub details: GovTalkDetails,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Body",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Body",
        default
    )]
    pub body: Option<GovTalkBody>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkHeader {
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}MessageDetails",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}MessageDetails"
    )]
    pub message_details: GovTalkMessageDetails,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}SenderDetails",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}SenderDetails",
        skip_serializing_if = "Option::is_none",
        default)]
    pub sender_details: Option<GovTalkSenderDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkMessageDetails {
    /// Min length: 4
    /// Max length: 32
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Class",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Class",
        default
    )]
    pub class: String,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Qualifier",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Qualifier"
    )]
    pub qualifier: GovTalkQualifier,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Function",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Function",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub function: Option<GovTalkFunction>,
    /// Max length: 32
    /// Regex: `[0-9][A-F]{0,32}`
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}TransactionID",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}TransactionID",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transaction_id: Option<String>,
    /// Max length: 32
    /// Regex: `[0-9][A-F]{0,32}`
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}AuditID",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}AuditID",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub audit_id: Option<String>,
    /// Max length: 32
    /// Regex: `[0-9][A-F]{0,32}`
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}CorrelationID",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}CorrelationID",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub correlation_id: Option<String>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}ResponseEndPoint",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}ResponseEndPoint",
        skip_serializing,
        default
    )]
    pub response_endpoint: Option<GovTalkResponseEndpoint>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Transformation",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transformation: Option<GovTalkTransformation>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}GatewayTest",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}GatewayTest",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub gateway_test: Option<i32>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}GatewayTimestamp",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}GatewayTimestamp",
        skip_serializing,
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    pub gateway_timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkResponseEndpoint {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "$attr:PollInterval", default = "poll_interval_default")]
    pub poll_interval: u32,
}

fn poll_interval_default() -> u32 {
    2
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkSenderDetails {
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}IDAuthentication",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}IDAuthentication",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub id_authentication: Option<GovTalkIDAuthentication>,
    /// Max length: 129
    /// Min length: 3
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}EmailAddress",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}EmailAddress",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub email_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkIDAuthentication {
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}SenderID",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}SenderID",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sender_id: Option<String>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Authentication",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Authentication"
    )]
    pub authentication: Vec<GovTalkAuthentication>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkAuthentication {
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Method",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Method"
    )]
    pub method: GovTalkAuthenticationMethod,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Role",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Role",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub role: Option<String>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Value",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Value"
    )]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkDetails {
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Keys",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Keys",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub keys: Option<GovTalkKeys>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}TargetDetails",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}TargetDetails",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub target_details: Option<GovTalkTargetDetails>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}GatewayValidation",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}GatewayValidation",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub gateway_validation: Option<GovTalkGatewayValidation>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}ChannelRouting",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}ChannelRouting",
        default
    )]
    pub channel_routing: Vec<GovTalkChannelRouting>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}GovTalkErrors",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}GovTalkErrors",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub errors: Option<GovTalkErrors>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkKeys {
    #[serde(
    rename = "{http://www.govtalk.gov.uk/CM/envelope}Key",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Key"
    )]
    keys: Vec<GovTalkKey>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkKey {
    #[serde(rename = "$value")]
    key: String,
    #[serde(rename = "$attr:Type")]
    key_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkTargetDetails {
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Organisation",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Organisation"
    )]
    organisations: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkGatewayValidation {
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Processed",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Processed"
    )]
    processed: GovTalkProcessed,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Result",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Result"
    )]
    result: GovTalkResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkChannelRouting {
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Channel",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Channel"
    )]
    channel: GovTalkChannel,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}ID",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}ID",
        default
    )]
    ids: Option<GovTalkChannelID>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Timestamp",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Timestamp",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_datetime_opt",
        default
    )]
    timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkChannel {
    #[serde(rename = "$value")]
    id: GovTalkChannelIDSelect,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Product",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Product",
        skip_serializing_if = "Option::is_none",
        default
    )]
    product: Option<String>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Version",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Version",
        skip_serializing_if = "Option::is_none",
        default
    )]
    version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GovTalkChannelIDSelect {
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}URI",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}URI"
    )]
    Uri(String),
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Name",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Name"
    )]
    Name(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkChannelID {
    #[serde(rename = "$value")]
    id: String,
    #[serde(rename = "$attr:Type")]
    id_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkErrors {
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Error",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Error"
    )]
    pub errors: Vec<GovTalkError>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovTalkError {
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}RaisedBy",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}RaisedBy"
    )]
    pub raised_by: String,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Number",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Number",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub number: Option<i32>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Type",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Type"
    )]
    pub error_type: GovTalkErrorType,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Text",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Text",
        default
    )]
    pub text: Vec<String>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Location",
        alias = "{http://www.govtalk.gov.uk/schemas/govtalk/govtalkheader}Location",
        default
    )]
    pub location: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GovTalkErrorType {
    #[serde(rename = "fatal")]
    Fatal,
    #[serde(rename = "recoverable")]
    Recoverable,
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "warning")]
    Warning
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GovTalkQualifier {
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "acknowledgement")]
    Acknowledgement,
    #[serde(rename = "response")]
    Response,
    #[serde(rename = "poll")]
    Poll,
    #[serde(rename = "error")]
    Error
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GovTalkFunction {
    #[serde(rename = "list")]
    List,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "submit")]
    Submit
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GovTalkTransformation {
    #[serde(rename = "XML")]
    Xml,
    #[serde(rename = "HTML")]
    Html,
    #[serde(rename = "text")]
    Text,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GovTalkAuthenticationMethod {
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "CHMD5")]
    CHMD5,
    #[serde(rename = "W3CSigned")]
    W3CSigned,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GovTalkProcessed {
    #[serde(rename = "no")]
    No,
    #[serde(rename = "yes")]
    Yes,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GovTalkResult {
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "fail")]
    Fail,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GovTalkBody {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/CompanyData-v3-4.xsd}CompanyDataRequest", skip_deserializing)]
    CompanyDataRequest(super::company_data::CompanyDataRequest),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyData", skip_serializing)]
    CompanyData(Box<super::company_data::CompanyData>),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope;http://xmlgw.companieshouse.gov.uk/v1-0/schema/MembersRegisterData-v1-0.xsd}MembersRegisterDataRequest", skip_deserializing)]
    MembersDataRequest(super::members_data::MembersDataRequest),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}MembersRegisterData", skip_serializing)]
    MembersData(super::members_data::MembersData),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/PaymentPeriods-v1-0.xsd}PaymentPeriodsRequest", skip_deserializing)]
    PaymentPeriodsRequest(super::payment_periods::PaymentPeriodsRequest),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}PaymentPeriods", skip_serializing)]
    PaymentPeriods(super::payment_periods::PaymentPeriods),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope;http://xmlgw.companieshouse.gov.uk/v1-0/schema/EReminders-v1-0.xsd}GetERemindersRequest", skip_deserializing)]
    GetERemindersRequest(super::e_reminders::GetERemindersRequest),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope;http://xmlgw.companieshouse.gov.uk/v1-0/schema/EReminders-v1-0.xsd}SetERemindersRequest", skip_deserializing)]
    SetERemindersRequest(super::e_reminders::SetERemindersRequest),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}EReminders", skip_serializing)]
    EReminders(super::e_reminders::EReminders),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/GetStatusAck-v1-1.xsd}StatusAck", skip_deserializing)]
    GetStatusAck {},
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}status", skip_serializing)]
    Status {},
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/GetSubmissionStatus-v2-9.xsd}GetSubmissionStatus", skip_deserializing)]
    GetSubmissionStatus(super::submission_status::GetSubmissionStatus),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}SubmissionStatus", skip_serializing)]
    SubmissionStatus(super::submission_status::SubmissionStatus),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/GetDocument-v1-1.xsd}GetDocument", skip_deserializing)]
    GetDocument(super::document::GetDocument),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Document", skip_serializing)]
    Document(super::document::Document),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/ChargeSearch-v2-8.xsd}ChargeSearch", skip_deserializing)]
    ChargeSearch(super::charge_search::ChargeSearch),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Charges", skip_serializing)]
    Charges(super::charge_search::Charges),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/Header;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/FormSubmission-v2-11.xsd}FormSubmission", skip_deserializing)]
    FormSubmission(Box<super::form_submission::FormSubmission>),
}
