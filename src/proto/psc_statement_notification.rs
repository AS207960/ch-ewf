use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct PSCStatementNotification {
    #[serde(rename = "$value")]
    pub notification: super::psc::PSCStatementNotificationType,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}RegisterEntryDate",
        serialize_with = "super::serialize_date"
    )]
    pub register_entry_date: Date<Utc>,
}