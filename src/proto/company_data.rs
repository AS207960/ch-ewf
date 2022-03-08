use chrono::prelude::*;

#[derive(Debug, Serialize)]
pub struct CompanyDataRequest {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyNumber")]
    pub company_number: u32,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_type: Option<super::base_types::CompanyType>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyAuthenticationCode")]
    pub company_authentication_code: String,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}MadeUpDate",
        serialize_with = "super::serialize_date"
    )]
    pub made_up_date: Date<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CompanyData {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyNumber")]
    pub company_number: u32,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyName")]
    pub company_name: String,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyCategory")]
    pub company_category: CompanyCategory,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Jurisdiction")]
    pub jurisdiction: CompanyJurisdiction,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}TradingOnMarket", default)]
    pub trading_on_market: bool,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}DTR5Applies", default)]
    pub dtr5_applies: bool,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}PSCExemptAsTradingOnRegulatedMarket", default)]
    pub psc_exempt_as_trading_on_regulated_market: bool,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}PSCExemptAsTradingOnUKRegulatedMarket", default)]
    pub psc_exempt_as_trading_on_uk_regulated_market: bool,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}PSCExemptAsSharesAdmittedOnMarket", default)]
    pub psc_exempt_as_shares_admitted_on_market: bool,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}MadeUpDate",
        deserialize_with = "super::deserialize_date"
    )]
    pub made_up_date: Date<Utc>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}NextDueDate",
        deserialize_with = "super::deserialize_date_opt",
        default
    )]
    pub next_due_date: Option<Date<Utc>>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}RegisteredOfficeAddress")]
    pub registered_office_address: super::base_types::UKAddress,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}SailAddress", default)]
    pub sail_address: Option<super::base_types::UKAddress>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}SailRecords", default)]
    pub sail_records: Vec<CompanyDataSAILRecord>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}SICCodes")]
    pub sic_codes: CompanyDataSICCodes,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Officers")]
    pub officers: CompanyDataOfficers,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}PSCs", default)]
    pub pscs: Option<CompanyDataPSCs>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}StatementOfCapital", default)]
    pub statement_of_captial: Option<super::base_types::StatementOfCapital>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Shareholdings", default)]
    pub shareholdings: Vec<CompanyDataShareholding>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Registers", default)]
    pub registers: Option<CompanyDataRegisters>,
}

#[derive(Debug, Deserialize)]
pub struct CompanyDataSAILRecord {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}RecordType")]
    pub record_type: super::base_types::RecordType,
}

#[derive(Debug, Deserialize)]
pub struct CompanyDataSICCodes {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}SICCode")]
    pub codes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CompanyDataOfficers {
    #[serde(rename = "$value")]
    pub officers: Vec<CompanyDataOfficer>,
}

#[derive(Debug, Deserialize)]
pub enum CompanyDataOfficer {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Director")]
    Director(Box<CompanyDataDirector>),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Secretary")]
    Secretary(CompanyDataSecretary),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Member")]
    Member(Box<CompanyDataMember>),
}

#[derive(Debug, Deserialize)]
pub struct CompanyDataDirector {
    #[serde(rename = "$value")]
    pub director_type: CompanyDataDirectorType,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}AppointmentDate",
        deserialize_with = "super::deserialize_date"
    )]
    pub appointment_date: Date<Utc>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}ResignationDate",
        deserialize_with = "super::deserialize_date_opt",
        default
    )]
    pub resignation_date: Option<Date<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CompanyDataSecretary {
    #[serde(rename = "$value")]
    pub secretary_type: CompanyDataSecretaryType,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}AppointmentDate",
        deserialize_with = "super::deserialize_date"
    )]
    pub appointment_date: Date<Utc>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}ResignationDate",
        deserialize_with = "super::deserialize_date_opt",
        default
    )]
    pub resignation_date: Option<Date<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CompanyDataMember {
    #[serde(rename = "$value")]
    pub member_type: CompanyDataMemberType,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}DesignatedInd")]
    pub designated: bool,
}

#[derive(Debug, Deserialize)]
pub enum CompanyDataDirectorType {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Person")]
    Person(Box<CompanyDataDirectorPersonType>),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Corporate")]
    Corporate(Box<super::base_types::CorporateOfficerType>),
}

#[derive(Debug, Deserialize)]
pub enum CompanyDataSecretaryType {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Person")]
    Person(Box<super::base_types::SecretaryPersonType<super::base_types::PersonReturnType>>),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Corporate")]
    Corporate(Box<super::base_types::CorporateOfficerType>),
}

#[derive(Debug, Deserialize)]
pub enum CompanyDataMemberType {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Person")]
    Person(CompanyDataMemberPersonType),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Corporate")]
    Corporate(super::base_types::CorporateOfficerType),
}

#[derive(Debug, Deserialize)]
pub struct CompanyDataDirectorPersonType {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}ResidentialAddress", default)]
    pub residential_address: Option<super::base_types::ResidentialAddressType>,
    #[serde(rename = "$value")]
    pub person: super::base_types::DirectorPersonType<super::base_types::PersonReturnType>,
}

#[derive(Debug, Deserialize)]
pub struct CompanyDataMemberPersonType {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}ResidentialAddress", default)]
    pub residential_address: Option<super::base_types::ResidentialAddressType>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}ServiceAddress")]
    pub service_address: super::base_types::ServiceAddressType,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}DOB",
        deserialize_with = "super::deserialize_date"
    )]
    pub date_of_birth: Date<Utc>,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CountryOfResidence", default)]
    pub country_of_residence: Option<String>,
    #[serde(rename = "$value")]
    pub person: super::base_types::PersonReturnType,
}

#[derive(Debug, Deserialize)]
pub enum CompanyCategory {
    Plc,
    #[serde(rename = "BYSHR")]
    ByShares,
    #[serde(rename = "BYGUAR")]
    ByGuarantee,
    #[serde(rename = "BYSHREXUNDSEC60")]
    BySharesExemptUnderSection60,
    #[serde(rename = "BYGUAREXUNDSEC60")]
    ByGuaranteeExemptUnderSection60,
    #[serde(rename = "UNLCWSHRCAP")]
    UnlimitedWithShareCapital,
    #[serde(rename = "UNLCWOSHRCAP")]
    UnlimitedWithoutShareCapital,
    Llp
}


#[derive(Debug, Serialize, Deserialize)]
pub enum CompanyJurisdiction {
    #[serde(rename = "EW")]
    EnglandAndWales,
    #[serde(rename = "SC")]
    Scotland,
    #[serde(rename = "WA")]
    Wales,
    #[serde(rename = "NI")]
    NorthernIreland,
    #[serde(rename = "EU")]
    EuropeanUnion,
    #[serde(rename = "UK")]
    UnitedKingdom,
    #[serde(rename = "EN")]
    England,
    #[serde(rename = "OTHER")]
    Other
}

#[derive(Debug, Deserialize)]
pub enum CompanyDataPSCs {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}CompanyStatement")]
    CompanyStatement(super::psc::CompanyLevelStatement),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}PSC")]
    PSCs(Vec<CompanyDataPSC>)
}

#[derive(Debug, Deserialize)]
pub enum CompanyDataPSC {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}PSCStatementNotification")]
    StatementNotification(super::psc::PSCLevelStatement),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}PSCLinkedStatementNotification")]
    LinkedStatementNotification(super::psc::PSCLinkedStatement),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}PSCNotification")]
    Notification(Box<CompanyDataPSCNotification>),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}SuperSecureIndividual")]
    SuperSecureIndividual(bool),
}

#[derive(Debug, Deserialize)]
pub struct CompanyDataPSCNotification {
    #[serde(rename = "$value")]
    pub notification: super::psc::PSCNotificationType,
    #[serde(rename = "$value")]
    pub nature_of_control: super::psc::PSCNatureOfControls,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}NotificationDate",
        deserialize_with = "super::deserialize_date"
    )]
    pub notification_date: Date<Utc>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}CessationDate",
        deserialize_with = "super::deserialize_date_opt",
        default
    )]
    pub cessation_date: Option<Date<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CompanyDataShareholding {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}ShareClass")]
    pub share_class: String,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}NumberHeld")]
    pub num_held: f64,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Shareholders")]
    pub shareholders: Vec<CompanyDataShareholder>
}

#[derive(Debug, Deserialize)]
pub struct CompanyDataShareholder {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Name")]
    pub name: CompanyDataShareholderName,
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Address")]
    pub address: super::base_types::BaseAddress
}

#[derive(Debug, Deserialize)]
pub enum CompanyDataShareholderName {
    #[serde(rename = "$value")]
    Name(CompanyDataShareholderNameParts),
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}AmalgamatedName")]
    AmalgamatedName(String)
}

#[derive(Debug, Deserialize)]
pub struct CompanyDataShareholderNameParts {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Surname")]
    pub surname: String,
    # [serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}Forename", default)]
    pub forenames: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct CompanyDataRegisters {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}HeldOnPublicRecord")]
    pub held_on_public_record: Vec<CompanyDataRegistersHeldOnPublicRecord>
}

#[derive(Debug, Deserialize)]
pub struct CompanyDataRegistersHeldOnPublicRecord {
    #[serde(rename = "{http://www.govtalk.gov.uk/CM/envelope}RegisterType")]
    pub register_type: super::base_types::RegisterType
}