#[derive(Debug, Serialize, Clone)]
pub struct CompanyIncorporation {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CompanyType")]
    pub company_type: CompanyType,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Cic",
        skip_serializing_if = "super::is_false"
    )]
    pub cic: bool,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}RegistersHeldOnPublicRecord",
        skip_serializing_if = "Option::is_none"
    )]
    pub registers_held_on_public_record: Option<RegistersHeldOnPublicRecord>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CountryOfIncorporation")]
    pub country_of_incorporation: CountryOfIncorporation,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}RegisteredOfficeAddress")]
    pub registered_office: super::base_types::UKAddress,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}DataMemorandum")]
    pub data_memorandum: bool,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Articles",
        skip_serializing_if = "Option::is_none"
    )]
    pub articles: Option<Articles>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}RestrictedArticles",
        skip_serializing_if = "super::is_false"
    )]
    pub restricted_articles: bool,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Appointment")]
    pub appointments: Vec<Appointment>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}PSCs")]
    pub pscs: PSCs,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}StatementOfCapital",
        skip_serializing_if = "Option::is_none"
    )]
    pub statement_of_capital: Option<super::base_types::StatementOfCapital>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Subscribers",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subscribers: Vec<Subscriber>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Guarantors",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub guarantors: Vec<Guarantor>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Authoriser")]
    pub authoriser: Authoriser,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}SameDay")]
    pub same_day: bool,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}SameName",
        skip_serializing_if = "super::is_false"
    )]
    pub same_name: bool,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}NameAuthorisation",
        skip_serializing_if = "super::is_false"
    )]
    pub name_authorisation: bool,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}RejectReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub reject_reference: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}SICCodes",
        skip_serializing_if = "Option::is_none"
    )]
    pub sic_codes: Option<super::base_types::SICCodes>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}StateSingleMemberCompany",
        skip_serializing_if = "Option::is_none"
    )]
    pub single_member_company: Option<bool>,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq, Copy)]
pub enum CompanyType {
    #[serde(rename = "PLC")]
    Plc,
    #[serde(rename = "BYSHR")]
    LimitedByShares,
    #[serde(rename = "BYGUAR")]
    LimitedByGuarantee,
    #[serde(rename = "BYGUAREXEMPT")]
    LimitedByGuaranteeExempt,
    #[serde(rename = "LLP")]
    Llp,
    #[serde(rename = "LLPDES")]
    LLPOnlyDesignated
}

#[derive(Debug, Serialize, Clone)]
pub struct RegistersHeldOnPublicRecord {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}LLPMembers",
        skip_serializing_if = "super::is_false"
    )]
    pub llp_members: bool,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}LLPMembersURA",
        skip_serializing_if = "super::is_false"
    )]
    pub llp_members_ura: bool,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Directors",
        skip_serializing_if = "super::is_false"
    )]
    pub directors: bool,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DirectorsURA",
        skip_serializing_if = "super::is_false"
    )]
    pub directors_ura: bool,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Secretaries",
        skip_serializing_if = "super::is_false"
    )]
    pub secretaries: bool,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Members",
        skip_serializing_if = "super::is_false"
    )]
    pub members: bool,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}PSC",
        skip_serializing_if = "Option::is_none"
    )]
    pub psc: Option<PSCRegister>
}

#[derive(Debug, Serialize, Clone)]
pub struct PSCRegister {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}StateNoObjection")]
    pub state_no_objection: bool
}

#[derive(Debug, Serialize, Clone)]
pub enum CountryOfIncorporation {
    #[serde(rename = "EW")]
    EnglandAndWales,
    #[serde(rename = "SC")]
    Scotland,
    #[serde(rename = "WA")]
    Wales,
    #[serde(rename = "NI")]
    NorthernIreland
}

#[derive(Debug, Serialize, Clone)]
pub enum Articles {
    #[serde(rename = "BYSHRMODEL")]
    ModelByShares,
    #[serde(rename = "BYGUARMODEL")]
    ModelByGuarantee,
    #[serde(rename = "PLCMODEL")]
    ModelPLC,
    #[serde(rename = "BYSHAREAMEND")]
    AmendedByShares,
    #[serde(rename = "BYGUARAMEND")]
    AmendedByGuarantee,
    #[serde(rename = "PLCAMEND")]
    AmendedPLC,
    #[serde(rename = "BESPOKE")]
    Bespoke,
}

#[derive(Debug, Serialize, Clone)]
pub struct Appointment {
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

#[derive(Debug, Serialize, Clone)]
pub enum PSCs {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}NoPSCStatement")]
    NoPSCStatement(NoPSCStatement),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}PSC")]
    PSCs(Vec<Psc>),
}

#[derive(Debug, Serialize, Clone)]
pub enum NoPSCStatement {
    #[serde(rename = "NO_INDIVIDUAL_OR_ENTITY_WITH_SIGNFICANT_CONTROL")]
    NoPSC
}

#[derive(Debug, Serialize, Clone)]
pub struct Psc {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}PSCNotification")]
    pub notification: PSCNotification
}

#[derive(Debug, Serialize, Clone)]
pub struct PSCNotification {
    #[serde(rename = "$value")]
    pub notification: PSCNotificationType,
    #[serde(rename = "$value")]
    pub nature_of_control: super::psc::PSCNatureOfControls,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PSCNotificationType {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Corporate",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Corporate"
    )]
    Corporate(super::psc::PSCCorporateEntity),
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}LegalPerson",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}LegalPerson"
    )]
    LegalPerson(super::psc::PSCLegalPerson),
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Individual",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Individual"
    )]
    Individual(Box<PSCIndividual>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PSCIndividual {
    #[serde(rename = "$value")]
    pub individual: super::psc::PSCIndividual,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ConsentStatement",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}ConsentStatement"
    )]
    pub consent_statement: bool
}


#[derive(Debug, Serialize, Clone)]
pub struct IncorporationPerson {
    #[serde(rename = "$value")]
    pub name: IncorporationPersonName,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Address")]
    pub address: super::base_types::BaseAddress,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Authentication")]
    pub authentication: Vec<super::base_types::PersonalAttribute>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}MemberClass",
        skip_serializing_if = "Option::is_none"
    )]
    pub member_class: Option<String>
}

#[derive(Debug, Serialize, Clone)]
pub struct Subscriber {
    #[serde(rename = "$value")]
    pub person: IncorporationPerson,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Shares")]
    pub shares: Vec<super::base_types::Allotment>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}MemorandumStatement",
        skip_serializing_if = "Option::is_none"
    )]
    pub memorandum_statement: Option<MemorandumStatement>
}

#[derive(Debug, Serialize, Clone)]
pub struct Guarantor {
    #[serde(rename = "$value")]
    pub person: IncorporationPerson,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}AmountGuaranteed")]
    pub amount_guaranteed: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}MemorandumStatement",
        skip_serializing_if = "Option::is_none"
    )]
    pub memorandum_statement: Option<MemorandumStatement>
}

#[derive(Debug, Serialize, Clone)]
pub enum MemorandumStatement {
    #[serde(rename = "Each subscriber to this memorandum of association wishes to form a company under the Companies Act 2006 and agrees to become a member of the company.")]
    MemberWithoutShares,
    #[serde(rename = "Each subscriber to this memorandum of association wishes to form a company under the Companies Act 2006 and agrees to become a member of the company and to take at least one share.")]
    MemberWithShares
}

#[derive(Debug, Serialize, Clone)]
pub enum IncorporationPersonName {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Person")]
    Person(super::base_types::Person),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Corporate")]
    Corporate(CorporateName)
}

#[derive(Debug, Serialize, Clone)]
pub enum Authoriser {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Agent")]
    Agent(Agent),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Solicitor")]
    Solicitor(AuthoriserType),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Member")]
    Member(AuthoriserType),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Subscribers")]
    Subscribers(AuthoriserSubscribers)
}

#[derive(Debug, Serialize, Clone)]
pub struct Agent {
    #[serde(rename = "$value")]
    pub authoriser: AuthoriserType,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Address")]
    pub address: super::base_types::BaseAddress,
}

#[derive(Debug, Serialize, Clone)]
pub struct AuthoriserType {
    #[serde(rename = "$value")]
    pub name: IncorporationPersonName,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Authentication")]
    pub authentication: Vec<super::base_types::PersonalAttribute>
}

#[derive(Debug, Serialize, Clone)]
pub struct CorporateName {
    #[serde(rename = "$value")]
    pub person_name: super::base_types::Person,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CorporateName")]
    pub corporate_name: String
}

#[derive(Debug, Serialize, Clone)]
pub struct AuthoriserSubscribers {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Subscriber")]
    pub subscribers: Vec<AuthoriserType>,
}
