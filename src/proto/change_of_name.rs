use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct ChangeOfName {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}MethodOfChange")]
    pub method_of_change: MethodOfChange,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ProposedCompanyName")]
    pub proposed_company_name: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}MeetingDate",
        serialize_with = "super::serialize_date_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub meeting_date: Option<Date<Utc>>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}SameDay",
        skip_serializing_if = "super::is_false"
    )]
    pub same_day: bool,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}NoticeGiven")]
    pub notice_given: bool,
}

#[derive(Debug, Serialize, Clone)]
pub enum MethodOfChange {
    #[serde(rename = "ARTICLES")]
    Articles,
    #[serde(rename = "RESOLUTION")]
    Resolution,
    #[serde(rename = "LLP")]
    Llp
}