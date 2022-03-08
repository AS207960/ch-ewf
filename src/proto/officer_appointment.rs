use chrono::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct OfficerAppointment {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}AppointmentDate",
        serialize_with = "super::serialize_date"
    )]
    pub appointment_date: Date<Utc>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ConsentToAct")]
    pub consent_to_act: bool,
    #[serde(rename = "$value")]
    pub appointment: AppointmentType
}

#[derive(Debug, Serialize, Clone)]
pub enum AppointmentType {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Director")]
    Director(super::base_types::DirectorAppointmentType),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Secretary")]
    Secretary(super::base_types::SecretaryAppointmentType),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Member")]
    Member(Box<super::base_types::MemberAppointmentType>)
}