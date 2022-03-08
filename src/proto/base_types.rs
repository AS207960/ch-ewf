use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum CompanyType {
    EW,
    SC,
    NI,
    R,
    OC,
    SO,
    NC
}

impl ToString for CompanyType {
    fn to_string(&self) -> String {
        match self {
            Self::EW => "",
            Self::SC => "SC",
            Self::NI => "NI",
            Self::R => "R",
            Self::OC => "OC",
            Self::SO => "SO",
            Self::NC => "NC",
        }.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum RecordType {
    #[serde(rename = "MEMBER")]
    Members,
    #[serde(rename = "DIR")]
    Directors,
    #[serde(rename = "DIRCONTRACT")]
    DirectorsServiceContracts,
    #[serde(rename = "DIRINDEM")]
    DirectorsIndemnities,
    #[serde(rename = "SEC")]
    Secretaries,
    #[serde(rename = "RESMEET")]
    ResolutionsAndMeetings,
    #[serde(rename = "DEB")]
    DebentureHolders,
    #[serde(rename = "CHARGEEWNI")]
    RegisterOfChargesEnglandWalesAndNorthernIreland,
    #[serde(rename = "CHARGESC")]
    RegisterOfChargesScotland,
    #[serde(rename = "OWNSHRPURCH")]
    OwnSharePurchaseContracts,
    #[serde(rename = "OWNSHRCAP")]
    OwnShareCapital,
    #[serde(rename = "INVEST")]
    InvestigationReports,
    #[serde(rename = "INTEREST")]
    RegisterOfInterests,
    #[serde(rename = "LLPMembers")]
    LLPMembers,
    #[serde(rename = "PSC")]
    PersonsOfSignificantControl,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum RegisterType {
    #[serde(rename = "DIRECTORS", alias = "DIR")]
    Directors,
    #[serde(rename = "DIRECTORSURA", alias = "DIRURA")]
    DirectorsUsualResidentialAddress,
    #[serde(rename = "SECRETARIES", alias = "SEC")]
    Secretaries,
    #[serde(rename = "MEMBERS")]
    Members,
    #[serde(rename = "LLPMEMBERS")]
    LLPMembers,
    #[serde(rename = "LLPMEMBERSURA")]
    LLPMembersUsualResidentialAddress,
    #[serde(rename = "PSC")]
    PersonsOfSignificantControl,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PartialDOBType {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Month",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Month",
    ))]
    pub month: u32,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Year",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Year",
    ))]
    pub year: u64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SICCodes {
    /// Min length 4; max length 5; pattern \[0-9\]*
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}SICCode")]
    pub codes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UKAddress {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Premise",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Premise"
    ))]
    pub premise: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Street",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Street",
    ), default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(rename(
        serialize =  "{http://xmlgw.companieshouse.gov.uk}Thoroughfare",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Thoroughfare",
    ), default, skip_serializing_if = "Option::is_none")]
    pub thoroughfare: Option<String>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}PostTown",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}PostTown"
    ))]
    pub post_town: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}County",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}County",
    ), default)]
    pub county: Option<String>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Country",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Country"
    ))]
    pub country: UKCountry,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Postcode",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Postcode",
    ), default, skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}CareofName",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}CareofName",
    ), default, skip_serializing_if = "Option::is_none")]
    pub care_of_name: Option<String>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}PoBox",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}PoBox",
    ), default, skip_serializing_if = "Option::is_none")]
    pub po_box: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UKCountry {
    #[serde(rename = "GB-ENG")]
    England,
    #[serde(rename = "GB-WLS")]
    Wales,
    #[serde(rename = "GB-SCT")]
    Scotland,
    #[serde(rename = "GB-NIR")]
    NorthernIreland,
    #[serde(rename = "GBR")]
    GreatBritain,
    #[serde(rename = "UNDEF")]
    Undefined
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaseAddress {
    #[serde(rename(
        serialize ="{http://xmlgw.companieshouse.gov.uk}Premise",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Premise"
    ))]
    pub premise: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Street",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Street",
    ), default, skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Thoroughfare",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Thoroughfare",
    ), default, skip_serializing_if = "Option::is_none")]
    pub thoroughfare: Option<String>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}PostTown",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}PostTown"
    ))]
    pub post_town: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}County",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}County",
    ), default, skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    #[serde(rename = "$value", default)]
    pub country: Option<AddressCountry>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Postcode",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Postcode",
    ), default, skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompanyAddress {
    #[serde(rename = "$value")]
    pub base_address: BaseAddress,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CareofName",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}CareofName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub care_of_name: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}PoBox",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}PoBox",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub po_box: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AddressCountry {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Country",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Country"
    )]
    Country(AddressCountryType),
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}OtherForeignCountry",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}OtherForeignCountry"
    )]
    OtherCountry(String)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AddressCountryType {
    #[serde(rename = "GB-ENG")]
    England,
    #[serde(rename = "GB-WLS")]
    Wales,
    #[serde(rename = "GB-SCT")]
    Scotland,
    #[serde(rename = "GB-NIR")]
    NorthernIreland,
    #[serde(rename = "GBR")]
    GreatBritain,
    #[serde(rename = "USA")]
    UnitedStatesOfAmerica,
    #[serde(rename = "IRL")]
    Ireland,
    #[serde(rename = "DEU")]
    Germany,
    #[serde(rename = "FRA")]
    France,
    #[serde(rename = "ITA")]
    Italy,
    #[serde(rename = "ESP")]
    Spain,
    #[serde(rename = "PRT")]
    Portugal,
    #[serde(rename = "NLD")]
    Netherlands,
    #[serde(rename = "POL")]
    Poland,
    #[serde(rename = "BEL")]
    Belgium,
    #[serde(rename = "NOR")]
    Norway,
    #[serde(rename = "SWE")]
    Sweden,
    #[serde(rename = "DNK")]
    Denmark,
    #[serde(rename = "AUS")]
    Australia,
    #[serde(rename = "NZL")]
    NewZealand,
    #[serde(rename = "CAN")]
    Canada,
    #[serde(rename = "ZAF")]
    SouthAfrica,
    #[serde(rename = "AUT")]
    Austria,
    #[serde(rename = "HRV")]
    Croatia,
    #[serde(rename = "CYP")]
    Cyprus,
    #[serde(rename = "CZE")]
    Czechia,
    #[serde(rename = "EST")]
    Estonia,
    #[serde(rename = "HUN")]
    Hungary,
    #[serde(rename = "GRC")]
    Greece,
    #[serde(rename = "LTU")]
    Lithuania,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatementOfCapital {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Capital",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Capital"
    )]
    pub capital: Vec<Capital>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Capital {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}TotalAmountUnpaid",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}TotalAmountUnpaid",
        default,
    )]
    pub total_amount_unpaid: f64,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}TotalNumberOfIssuedShares",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}TotalNumberOfIssuedShares",
        default
    )]
    pub total_issued_shares: f64,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ShareCurrency",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}ShareCurrency",
    )]
    pub share_currency: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}TotalAggregateNominalValue",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}TotalAggregateNominalValue",
        default
    )]
    pub total_aggregate_value: f64,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Shares",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Shares"
    )]
    pub shares: Vec<Share>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Share {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ShareClass",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}ShareClass"
    )]
    pub share_class: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}PrescribedParticulars",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}PrescribedParticulars",
        default
    )]
    pub particulars: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}NumShares",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}NumShares"
    )]
    pub num_shares: f64,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}AggregateNominalValue",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}AggregateNominalValue"
    )]
    pub aggregate_value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StocksOrSharesHeld {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}SharesHeld")]
    SharesHeld(SharesHeld),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}StocksHeld")]
    StocksHeld(StocksHeld)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shares {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}NumShares")]
    pub num_shares: f64,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ShareClass")]
    pub share_class: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ShareReference", default)]
    pub share_reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SharesHeld {
    #[serde(rename = "$value")]
    pub shares: Shares,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}AmountPaidUp")]
    pub paid_up: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StocksHeld {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}StockClass")]
    pub stock_class: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}AmountHeld")]
    pub amount_held: f64,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Currency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompanyMember {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}MemberClass",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub member_class: Option<String>,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Name")]
    pub name: CompanyMemberName,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Address")]
    pub address: BaseAddress
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CompanyMemberName {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}CorporateName")]
    CorporateName(String),
    #[serde(rename = "$value")]
    Person(Person)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Forename")]
    pub forename: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Surname")]
    pub surname: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonReturnType {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Title",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Title",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Forename",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Forename",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub forenames: Vec<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Surname",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Surname"
    )]
    pub surname: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonType {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Title",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Title",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Surname",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Surname"
    )]
    pub surname: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Forename",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Forename",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub forename: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}OtherForenames",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}OtherForenames",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub other_forenames: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonType2 {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Title",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Title",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Forename",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Forename",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub forename: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}OtherForenames",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}OtherForenames",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub other_forenames: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Surname",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Surname"
    )]
    pub surname: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceAddressType {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}SameAsRegisteredOffice",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}SameAsRegisteredOffice"
    )]
    SameAsRegisteredOffice(bool),
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Address",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Address"
    )]
    Address(Box<CompanyAddress>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResidentialAddressType {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}SameAsServiceAddress",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}SameAsServiceAddress"
    )]
    SameAsServiceAddress(bool),
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Address",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Address"
    )]
    Address(ResidentialBaseAddress)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResidentialBaseAddress {
    #[serde(rename = "$value")]
    pub address: BaseAddress,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}SecureAddressInd",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}SecureAddressInd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secure_address: Option<bool>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectorPersonType<P> {
    #[serde(rename = "$value")]
    pub person: P,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ServiceAddress",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}ServiceAddress"
    )]
    pub service_address: ServiceAddressType,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DOB",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}DOB",
        deserialize_with = "super::deserialize_date",
        serialize_with = "super::serialize_date",
    )]
    pub date_of_birth: Date<Utc>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Nationality",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Nationality"
    )]
    pub nationality: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Occupation",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Occupation"
    )]
    pub occupation: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CountryOfResidence",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}CountryOfResidence",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub country_of_residence: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}PreviousNames",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}PreviousNames",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub previous_names: Vec<PreviousNameType>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ResidentialAddress",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}ResidentialAddress"
    )]
    pub residential_address: ResidentialAddressType
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecretaryPersonType<P> {
    #[serde(rename = "$value")]
    pub person: P,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ServiceAddress",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}ServiceAddress"
    )]
    pub service_address: ServiceAddressType,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}PreviousNames",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}PreviousNames",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub previous_names: Vec<PreviousNameType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberPersonType<P> {
    #[serde(rename = "$value")]
    pub person: P,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ServiceAddress",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}ServiceAddress"
    )]
    pub service_address: ServiceAddressType,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}DOB",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}DOB",
        deserialize_with = "super::deserialize_date",
        serialize_with = "super::serialize_date",
    )]
    pub date_of_birth: Date<Utc>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CountryOfResidence",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}CountryOfResidence",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub country_of_residence: Option<String>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}PreviousNames",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}PreviousNames",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub previous_names: Vec<PreviousNameType>,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ResidentialAddress",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}ResidentialAddress"
    )]
    pub residential_address: ResidentialAddressType
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreviousNameType {
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Forename",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Forename",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub forename: Option<String>,
    #[serde(
        rename = "{http://www.govtalk.gov.uk/CM/envelope}Surname",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Surname"
    )]
    pub surname: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CorporateOfficerType {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CorporateName",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}CorporateName"
    )]
    pub corporate_name: String,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}Address",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}Address"
    )]
    pub address: CompanyAddress,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}CompanyIdentification",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}CompanyIdentification",
        default,
    )]
    pub company_identification: Option<CompanyIdentification>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CompanyIdentification {
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}UK",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}UK"
    )]
    UK {
        #[serde(
            rename = "{http://xmlgw.companieshouse.gov.uk}RegistrationNumber",
            alias = "{http://www.govtalk.gov.uk/CM/envelope}RegistrationNumber"
        )]
        registration_number: String,
    },
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}NonUK",
        alias = "{http://www.govtalk.gov.uk/CM/envelope}NonUK"
    )]
    NonUK {
        #[serde(
            rename = "{http://xmlgw.companieshouse.gov.uk}PlaceRegistered",
            alias = "{http://www.govtalk.gov.uk/CM/envelope}PlaceRegistered"
        )]
        place_registered: Option<String>,
        #[serde(
            rename = "{http://xmlgw.companieshouse.gov.uk}RegistrationNumber",
            alias = "{http://www.govtalk.gov.uk/CM/envelope}RegistrationNumber"
        )]
        registration_number: Option<String>,
        #[serde(
            rename = "{http://xmlgw.companieshouse.gov.uk}LawGoverned",
            alias = "{http://www.govtalk.gov.uk/CM/envelope}LawGoverned"
        )]
        governing_law: String,
        #[serde(
            rename = "{http://xmlgw.companieshouse.gov.uk}LegalForm",
            alias = "{http://www.govtalk.gov.uk/CM/envelope}LegalForm"
        )]
        legal_form: String,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CorporateOfficerAppointmentType {
    #[serde(
        rename = "$value",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Person>,
    #[serde(rename = "$value")]
    pub corporate_officer: CorporateOfficerType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DirectorAppointmentType {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Person")]
    Person(Box<DirectorPersonType<PersonType2>>),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Corporate")]
    Corporate(Box<CorporateOfficerAppointmentType>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SecretaryAppointmentType {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Person")]
    Person(SecretaryPersonType<PersonType2>),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Corporate")]
    Corporate(Box<CorporateOfficerAppointmentType>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MemberAppointmentTypeInner {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Person")]
    Person(Box<MemberPersonType<PersonType2>>),
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}Corporate")]
    Corporate(Box<CorporateOfficerAppointmentType>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberAppointmentType {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}DesignatedInd")]
    pub designated: bool,
    #[serde(rename = "$value")]
    pub inner: MemberAppointmentTypeInner
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Allotment {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ShareClass")]
    pub share_class: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}NumShares")]
    pub num_shares: f64,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}AmountPaidDuePerShare")]
    pub amount_paid_due_per_share: f64,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}AmountUnpaidPerShare")]
    pub amount_unpaid_per_share: f64,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ShareCurrency")]
    pub share_currency: String,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}ShareValue")]
    pub share_value: f64,
    #[serde(
        rename = "{http://xmlgw.companieshouse.gov.uk}ShareReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub share_reference: Option<String>
}

#[derive(Debug, Serialize, Clone)]
pub struct PersonalAttribute {
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}PersonalAttribute")]
    pub personal_attribute: PersonalAttributeType,
    #[serde(rename = "{http://xmlgw.companieshouse.gov.uk}PersonalData")]
    pub personal_data: String
}

#[derive(Debug, Serialize, Clone)]
pub enum PersonalAttributeType {
    #[serde(rename = "BIRTOWN")]
    BirthTown,
    #[serde(rename = "TEL")]
    Telephone,
    #[serde(rename = "NATINS")]
    NationalInsurance,
    #[serde(rename = "PASSNO")]
    PassportNumber,
    #[serde(rename = "MUM")]
    MothersMaiden,
    #[serde(rename = "DAD")]
    FathersForename
}