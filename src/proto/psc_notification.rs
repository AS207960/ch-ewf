use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct PSCNotification {
    #[serde(rename = "$value")]
    pub notification: super::psc::PSCNotificationType,
    #[serde(rename = "$value")]
    pub nature_of_control: super::psc::PSCNatureOfControls,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}NotificationDate",
        serialize_with = "super::serialize_date"
    )]
    pub notification_date: Date<Utc>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}RegisterEntryDate",
        serialize_with = "super::serialize_date"
    )]
    pub register_entry_date: Date<Utc>,
}