use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct PSCStatementWithdrawal {
    #[serde(rename = "$value")]
    pub notification: super::psc::PSCStatementNotificationType,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}WithdrawalDate",
        serialize_with = "super::serialize_date"
    )]
    pub withdrawal_date: Date<Utc>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}RestrictionsNoticeWithdrawalReason",
        skip_serializing_if = "Option::is_none"
    )]
    pub restrictions_notice_withdrawal_reason: Option<RestrictionsNoticeWithdrawalReason>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}RegisterEntryDate",
        serialize_with = "super::serialize_date"
    )]
    pub register_entry_date: Date<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub enum RestrictionsNoticeWithdrawalReason {
    #[serde(rename = "RESTRICTIONS_NOTICE_WITHDRAWN_BY_COMPANY")]
    WithdrawnByCompany,
    #[serde(rename = "RESTRICTIONS_NOTICE_WITHDRAWN_BY_COURT_ORDER")]
    WithdrawnByCourtOrder
}