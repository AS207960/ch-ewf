use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CompanyLevelStatement {
    #[serde(rename = "NO_INDIVIDUAL_OR_ENTITY_WITH_SIGNFICANT_CONTROL")]
    NoSignificantControl,
    #[serde(rename = "STEPS_TO_FIND_PSC_NOT_YET_COMPLETED")]
    StepsNotCompleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum PSCLevelStatement {
    #[serde(rename = "PSC_EXISTS_BUT_NOT_IDENTIFIED")]
    ExistsButNotIdentified,
    #[serde(rename = "PSC_DETAILS_NOT_CONFIRMED")]
    DetailsNotConfirmed,
    #[serde(rename = "PSC_CONTACTED_BUT_NO_RESPONSE")]
    ContactedButNoResponse,
    #[serde(rename = "RESTRICTIONS_NOTICE_ISSUED_TO_PSC")]
    RestrictionNoticeIssued,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PSCLinkedStatement {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Statement",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Statement"
    ))]
    pub statement: PSCLinkedStatementType,
    #[serde(rename = "$value")]
    pub entity: PSCLinkedStatementEntity
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum PSCLinkedStatementType {
    #[serde(rename = "PSC_HAS_FAILED_TO_CONFIRM_CHANGED_DETAILS")]
    FailedToConfirmChangedDetails
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PSCLinkedStatementEntity {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}SuperSecureIndividual",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}SuperSecureIndividual"
    ))]
    SuperSecureIndividual(bool),
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Corporate",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Corporate"
    ))]
    Corporate(LinkedStatementCorporate),
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}LegalPerson",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}LegalPerson"
    ))]
    LegalPerson(LinkedStatementLegalPerson),
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Individual",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Individual"
    ))]
    Individual(PSCIdentification),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LinkedStatementCorporate {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}CorporateName",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}CorporateName"
    ))]
    pub corporate_name: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LinkedStatementLegalPerson {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}LegalPersonName",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}LegalPersonName"
    ))]
    pub legal_person_name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PSCIdentification {
    #[serde(rename = "$value")]
    pub name: super::base_types::PersonType,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}PartialDOB",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}PartialDOB",
    ), default, skip_serializing_if = "Option::is_none")]
    pub partial_dob: Option<super::base_types::PartialDOBType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PSCIndividual<P> {
    #[serde(rename = "$value")]
    pub person: P,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}ServiceAddress",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}ServiceAddress"
    ))]
    pub service_address: super::base_types::ServiceAddressType,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}DOB",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}DOB",
    ), deserialize_with = "super::deserialize_date", serialize_with = "super::serialize_date")]
    pub date_of_birth: Date<Utc>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Nationality",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Nationality"
    ))]
    pub nationality: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}CountryOfResidence",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}CountryOfResidence",
    ), default)]
    pub country_of_residence: Option<String>,
    #[serde(rename(
        serialize = "{http://www.govtalk.gov.uk/CM/envelope}ResidentialAddress",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}ResidentialAddress",
    ))]
    pub residential_address: super::base_types::ResidentialAddressType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PSCNotificationType<P> {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Corporate",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Corporate"
    ))]
    Corporate(PSCCorporateEntity),
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}LegalPerson",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}LegalPerson"
    ))]
    LegalPerson(PSCLegalPerson),
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Individual",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Individual"
    ))]
    Individual(Box<PSCIndividual<P>>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PSCStatementNotificationType {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}CompanyStatement",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}CompanyStatement"
    ))]
    Company(CompanyLevelStatement),
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}PSCStatement",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}PSCStatement"
    ))]
    Psc(PSCLevelStatement),
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}PSCLinkedStatement",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}PSCLinkedStatement"
    ))]
    PscLinked(PSCLinkedStatement),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PSCCorporateEntity {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}CorporateName",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}CorporateName"
    ))]
    pub corporate_name: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Address",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Address"
    ))]
    pub address: super::base_types::CompanyAddress,
    #[serde(rename(
        serialize = "{http://www.govtalk.gov.uk/CM/envelope}PSCCompanyIdentification",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}PSCCompanyIdentification"
    ))]
    pub company_identification: PSCCorporateIdentification,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PSCCorporateIdentification {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}PSCPlaceRegistered",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}PSCPlaceRegistered",
    ), default, skip_serializing_if = "Option::is_none")]
    pub place_registered: Option<String>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}PSCRegistrationNumber",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}PSCRegistrationNumber",
    ), default, skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}LawGoverned",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}LawGoverned"
    ))]
    pub law_governed: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}LegalForm",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}LegalForm"
    ))]
    pub legal_form: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}CountryOrState",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}CountryOrState"
    ))]
    pub country_or_state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PSCLegalPerson {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}LegalPersonName",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}LegalPersonName"
    ))]
    pub name: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}Address",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}Address"
    ))]
    pub address: super::base_types::CompanyAddress,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}LegalPersonIdentification",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}LegalPersonIdentification"
    ))]
    pub legal_person_identification: PSCLegalPersonIdentification
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PSCLegalPersonIdentification {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}LawGoverned",
        deserialize = "{http://xmlgw.companieshouse.gov.uk}LawGoverned"
    ))]
    pub law_governed: String,
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}LegalForm",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}LegalForm",
    ))]
    pub legal_form: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PSCNatureOfControls {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}NatureOfControls",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}NatureOfControls"
    ))]
    NatureOfControls(NatureOfControls),
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}LLPNatureOfControls",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}LLPNatureOfControls"
    ))]
    LLPNatureOfControls(LLPNatureOfControls),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NatureOfControls {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}NatureOfControl",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}NatureOfControl"
    ))]
    pub nature_of_control: Vec<NatureOfControlType>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LLPNatureOfControls {
    #[serde(rename(
        serialize = "{http://xmlgw.companieshouse.gov.uk}NatureOfControl",
        deserialize = "{http://www.govtalk.gov.uk/CM/envelope}NatureOfControl"
    ))]
    pub nature_of_control: Vec<LLPNatureOfControlType>
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum NatureOfControlType {
    #[serde(rename = "OWNERSHIPOFSHARES_25TO50PERCENT")]
    OwnershipOfShares25To50,
    #[serde(rename = "OWNERSHIPOFSHARES_50TO75PERCENT")]
    OwnershipOfShares50To75,
    #[serde(rename = "OWNERSHIPOFSHARES_75TO100PERCENT")]
    OwnershipOfShares75To100,
    #[serde(rename = "OWNERSHIPOFSHARES_25TO50PERCENT_AS_TRUST")]
    OwnershipOfShares25To50AsTrust,
    #[serde(rename = "OWNERSHIPOFSHARES_50TO75PERCENT_AS_TRUST")]
    OwnershipOfShares50To75AsTrust,
    #[serde(rename = "OWNERSHIPOFSHARES_75TO10PERCENT_AS_TRUST")]
    OwnershipOfShares75To100AsTrust,
    #[serde(rename = "OWNERSHIPOFSHARES_25TO50PERCENT_AS_FIRM")]
    OwnershipOfShares25To50AsFirm,
    #[serde(rename = "OWNERSHIPOFSHARES_50TO75PERCENT_AS_FIRM")]
    OwnershipOfShares50To75AsFirm,
    #[serde(rename = "OWNERSHIPOFSHARES_75TO100PERCENT_AS_FIRM")]
    OwnershipOfShares75To100AsFirm,
    #[serde(rename = "VOTINGRIGHTS_25TO50PERCENT")]
    VotingRights25To50,
    #[serde(rename = "VOTINGRIGHTS_50TO75PERCENT")]
    VotingRights50To75,
    #[serde(rename = "VOTINGRIGHTS_75TO100PERCENT")]
    VotingRights75To100,
    #[serde(rename = "VOTINGRIGHTS_25TO50PERCENT_AS_TRUST")]
    VotingRights25To50AsTrust,
    #[serde(rename = "VOTINGRIGHTS_50TO75PERCENT_AS_TRUST")]
    VotingRights50To75AsTrust,
    #[serde(rename = "VOTINGRIGHTS_75TO100PERCENT_AS_TRUST")]
    VotingRights75To100AsTrust,
    #[serde(rename = "VOTINGRIGHTS_25TO50PERCENT_AS_FIRM")]
    VotingRights25To50AsFirm,
    #[serde(rename = "VOTINGRIGHTS_50TO75PERCENT_AS_FIRM")]
    VotingRights50To75AsFirm,
    #[serde(rename = "VOTINGRIGHTS_75TO100PERCENT_AS_FIRM")]
    VotingRights75To100AsFirm,
    #[serde(rename = "RIGHTTOAPPOINTANDREMOVEDIRECTORS")]
    RightToAppointAndRemoveDirectors,
    #[serde(rename = "RIGHTTOAPPOINTANDREMOVEDIRECTORS_AS_TRUST")]
    RightToAppointAndRemoveDirectorsAsTrust,
    #[serde(rename = "RIGHTTOAPPOINTANDREMOVEDIRECTORS_AS_FIRM")]
    RightToAppointAndRemoveDirectorsAsFirm,
    #[serde(rename = "SIGINFLUENCECONTROL")]
    SignificantInfluence,
    #[serde(rename = "SIGINFLUENCECONTROL_AS_TRUST")]
    SignificantInfluenceAsTrust,
    #[serde(rename = "SIGINFLUENCECONTROL_AS_FIRM")]
    SignificantInfluenceAsFirm,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum LLPNatureOfControlType {
    #[serde(rename = "RIGHTTOSHARESURPLUSASSETS_25TO50PERCENT")]
    RightToSurplusAssets25To50,
    #[serde(rename = "RIGHTTOSHARESURPLUSASSETS_50TO75PERCENT")]
    RightToSurplusAssets50To75,
    #[serde(rename = "RIGHTTOSHARESURPLUSASSETS_75TO100PERCENT")]
    RightToSurplusAssets75To100,
    #[serde(rename = "RIGHTTOSHARESURPLUSASSETS_25TO50PERCENT_AS_TRUST")]
    RightToSurplusAssets25To50AsTrust,
    #[serde(rename = "RIGHTTOSHARESURPLUSASSETS_50TO75PERCENT_AS_TRUST")]
    RightToSurplusAssets50To75AsTrust,
    #[serde(rename = "RIGHTTOSHARESURPLUSASSETS_75TO100PERCENT_AS_TRUST")]
    RightToSurplusAssets75To100AsTrust,
    #[serde(rename = "RIGHTTOSHARESURPLUSASSETS_25TO50PERCENT_AS_FIRM")]
    RightToSurplusAssets25To50AsFirm,
    #[serde(rename = "RIGHTTOSHARESURPLUSASSETS_50TO75PERCENT_AS_FIRM")]
    RightToSurplusAssets50To75AsFirm,
    #[serde(rename = "RIGHTTOSHARESURPLUSASSETS_75T0O10PERCENT_AS_FIRM")]
    RightToSurplusAssets75To100AsFirm,
    #[serde(rename = "VOTINGRIGHTS_25TO50PERCENT")]
    VotingRights25To50,
    #[serde(rename = "VOTINGRIGHTS_50TO75PERCENT")]
    VotingRights50To75,
    #[serde(rename = "VOTINGRIGHTS_75TO100PERCENT")]
    VotingRights75To100,
    #[serde(rename = "VOTINGRIGHTS_25TO50PERCENT_AS_TRUST")]
    VotingRights25To50AsTrust,
    #[serde(rename = "VOTINGRIGHTS_50TO75PERCENT_AS_TRUST")]
    VotingRights50To75AsTrust,
    #[serde(rename = "VOTINGRIGHTS_75TO100PERCENT_AS_TRUST")]
    VotingRights75To100AsTrust,
    #[serde(rename = "VOTINGRIGHTS_25TO50PERCENT_AS_FIRM")]
    VotingRights25To50AsFirm,
    #[serde(rename = "VOTINGRIGHTS_50TO75PERCENT_AS_FIRM")]
    VotingRights50To75AsFirm,
    #[serde(rename = "VOTINGRIGHTS_75TO100PERCENT_AS_FIRM")]
    VotingRights75To100AsFirm,
    #[serde(rename = "RIGHTTOAPPOINTANDREMOVEMEMBERS")]
    RightToAppointAndRemoveMembers,
    #[serde(rename = "RIGHTTOAPPOINTANDREMOVEMEMBERS_AS_TRUST")]
    RightToAppointAndRemoveMembersAsTrust,
    #[serde(rename = "RIGHTTOAPPOINTANDREMOVEMEMBERS_AS_FIRM")]
    RightToAppointAndRemoveMembersAsFirm,
    #[serde(rename = "SIGINFLUENCECONTROL")]
    SignificantInfluence,
    #[serde(rename = "SIGINFLUENCECONTROL_AS_TRUST")]
    SignificantInfluenceAsTrust,
    #[serde(rename = "SIGINFLUENCECONTROL_AS_FIRM")]
    SignificantInfluenceAsFirm,
}