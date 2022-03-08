use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct FormSubmission {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/Header}FormHeader")]
    pub form_header: FormHeader,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/Header}DateSigned",
        serialize_with = "super::serialize_date"
    )]
    pub date_signed: Date<Utc>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/Header}Form")]
    pub form: Form,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/Header}AdditionalInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_information: Option<AdditionalInformation>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/Header}Document",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub documents: Vec<Document>
}

#[derive(Debug, Serialize)]
pub struct FormHeader {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/Header}CompanyNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_number: Option<u32>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/Header}CompanyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_type: Option<super::base_types::CompanyType>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/Header}CompanyName")]
    pub company_name: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/Header}CompanyAuthenticationCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_authentication_code: Option<String>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/Header}PackageReference")]
    pub package_reference: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/Header}Language")]
    pub language: SubmissionLanguage,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/Header}FormIdentifier")]
    pub form_identifier: String,
    /// Fixed length: 6
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/Header}SubmissionNumber")]
    pub submission_number: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/Header}ContactName",
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_name: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/Header}ContactNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_number: Option<String>,
    /// Max length: 25
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/Header}CustomerReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_reference: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum SubmissionLanguage {
    #[serde(rename = "EN")]
    English,
    #[serde(rename = "CY")]
    Welsh
}

#[derive(Debug, Serialize)]
pub struct Document {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/Header}Data")]
    pub data: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/Header}Date",
        serialize_with = "super::serialize_date_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub date: Option<Date<Utc>>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk/Header}Filename",
        skip_serializing_if = "Option::is_none"
    )]
    pub filename: Option<String>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/Header}ContentType")]
    pub content_type: ContentType,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk/Header}Category")]
    pub category: Category,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub enum ContentType {
    #[serde(rename = "application/vnd.hp-pcl")]
    Pcl,
    #[serde(rename = "application/xml")]
    Xml,
    #[serde(rename = "application/pdf")]
    Pdf
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub enum Category {
    #[serde(rename = "MEMARTS")]
    MemorandumAndArticles,
    #[serde(rename = "MEM")]
    Memorandum,
    #[serde(rename = "ARTS")]
    Articles,
    #[serde(rename = "ACCOUNTS")]
    Accounts,
    #[serde(rename = "SUPPNAMEAUTH")]
    NameAuthentication,
    #[serde(rename = "SUPPEXISTNAME")]
    NameExisting,
    #[serde(rename = "DEED")]
    Deed,
    #[serde(rename = "DEEDSUPP")]
    DeedSupplemental,
    CIC36
}

#[derive(Debug, Serialize)]
pub enum Form {
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/ConfirmationStatement-v1-2.xsd}ConfirmationStatement")]
    ConfirmationStatement(super::confirmation_statement::ConfirmationStatement),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/ChangeAccountingReferenceDate-v2-7.xsd}ChangeAccountingReferenceDate")]
    ChangeAccountingReferenceDate(super::accounting_reference_date::ChangeAccountingReferenceDate),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/ChangeOfName-v2-6.xsd}ChangeOfName")]
    ChangeOfName(super::change_of_name::ChangeOfName),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/ChangeRegisteredOfficeAddress-v2-5.xsd}ChangeRegisteredOfficeAddress")]
    ChangeRegisteredOffice(super::change_registered_office::ChangeRegisteredOfficeAddress),
    // This appears to only apply to the Companies Act 1985, won't implement further unless there's some need to
    // #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/IncreaseNominalCapital-v2-6.xsd}IncreaseNominalCapital")]
    // IncreaseNominalCapital(super::increase_nominal_capital::IncreaseNominalCapital),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/MembersRegisterElectOrWithdraw-v1-0.xsd}MembersRegisterElectOrWithdraw")]
    MembersRegisterElectOrWithdraw(super::members_register::MembersRegisterElectOrWithdraw),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/MembersRegisterUpdate-v1-0.xsd}MembersRegisterUpdate")]
    MembersRegisterUpdate(super::members_register_update::MembersRegisterUpdate),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/OfficerAppointment-v2-8.xsd}OfficerAppointment")]
    OfficerAppointment(super::officer_appointment::OfficerAppointment),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/OfficerChangeDetails-v2-9.xsd}OfficerChangeDetails")]
    OfficerChangeDetails(super::officer_change::OfficerChangeDetails),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/OfficerResignation-v2-6.xsd}OfficerResignation")]
    OfficerResignation(super::officer_resignation::OfficerResignation),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/PSCCessation-v1-1.xsd}PSCCessation")]
    PSCCessation(super::psc_cessation::PSCCessation),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/PSCChangeDetails-v1-1.xsd}PSCChangeDetails")]
    PSCChangeDetails(super::psc_change_details::PSCChangeDetails),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/PSCNotification-v1-1.xsd}PSCNotification")]
    PSCNotification(super::psc_notification::PSCNotification),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/PSCStatementNotification-v1-1.xsd}PSCStatementNotification")]
    PSCStatementNotification(super::psc_statement_notification::PSCStatementNotification),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/PSCStatementWithdrawal-v1-1.xsd}PSCStatementWithdrawal")]
    PSCStatementWithdrawal(super::psc_statement_withdrawal::PSCStatementWithdrawal),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/RecordChangeofLocation-v2-7.xsd}RecordChangeOfLocation")]
    RecordChangeOfLocation(super::change_of_location::RecordChangeOfLocation),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/RegisterElectOrWithdraw-v1-0.xsd}RegisterElectOrWithdraw")]
    RegisterElectOrWithdraw(super::register_elect_or_withdraw::RegisterElectOrWithdraw),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/ReturnofAllotmentShares-v3-0.xsd}ReturnofAllotmentShares")]
    ReturnOfAllotmentShares(super::return_allotment_shares::ReturnOfAllotmentShares),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/SailAddress-v2-6.xsd}SailAddress")]
    SAILAddress(super::sail_address::SAILAddress),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/CompanyIncorporation-v3-6.xsd}CompanyIncorporation")]
    CompanyIncorporation(Box<super::company_incorporation::CompanyIncorporation>),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/ChargeRegistration-v2-9.xsd}ChargeRegistration")]
    ChargeRegistration(super::charge_registration::ChargeRegistration),
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/ChargeUpdate-v2-8.xsd}ChargeUpdate")]
    ChargeUpdate(super::charge_update::ChargeUpdate),
}

#[derive(Debug, Serialize)]
pub enum AdditionalInformation {
    #[serde(rename="{http://xmlgw.companieshouse.gov.uk/HMRC;http://xmlgw.companieshouse.gov.uk/v1-0/schema/forms/additionalInformation/HMRC-v1-0.xsd}CorporationTaxInformation")]
    CorporationTaxInformation(super::corporation_tax_information::CorporationTaxInformation)
}