use std::convert::{TryFrom, TryInto};
use super::{ch_ewf_grpc, proto};
use super::grpc::proto_to_chrono;

impl TryFrom<ch_ewf_grpc::psc::Notification> for proto::psc::PSCNotificationType<proto::base_types::PersonType2> {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::psc::Notification) -> Result<Self, Self::Error> {
        Ok(match value.psc {
            Some(ch_ewf_grpc::psc::notification::Psc::Corporate(c)) => {
                proto::psc::PSCNotificationType::Corporate(c.try_into()?)
            }
            Some(ch_ewf_grpc::psc::notification::Psc::LegalPerson(c)) => {
                proto::psc::PSCNotificationType::LegalPerson(c.try_into()?)
            }
            Some(ch_ewf_grpc::psc::notification::Psc::Individual(c)) => {
                proto::psc::PSCNotificationType::Individual(Box::new(c.try_into()?))
            }
            None => return Err(tonic::Status::invalid_argument("PSC entity number"))
        })
    }
}

impl TryFrom<ch_ewf_grpc::psc::Notification> for proto::company_incorporation::PSCNotificationType {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::psc::Notification) -> Result<Self, Self::Error> {
        Ok(match value.psc {
            Some(ch_ewf_grpc::psc::notification::Psc::Corporate(c)) => {
                proto::company_incorporation::PSCNotificationType::Corporate(c.try_into()?)
            }
            Some(ch_ewf_grpc::psc::notification::Psc::LegalPerson(c)) => {
                proto::company_incorporation::PSCNotificationType::LegalPerson(c.try_into()?)
            }
            Some(ch_ewf_grpc::psc::notification::Psc::Individual(c)) => {
                if !c.consent_statement {
                    return Err(tonic::Status::invalid_argument("PSC consent required"));
                }

                proto::company_incorporation::PSCNotificationType::Individual(Box::new(proto::company_incorporation::PSCIndividual {
                    individual: c.try_into()?,
                    consent_statement: true,
                }))
            }
            None => return Err(tonic::Status::invalid_argument("PSC entity number"))
        })
    }
}

impl TryFrom<ch_ewf_grpc::psc::CorporateEntity> for proto::psc::PSCCorporateEntity {
    type Error = tonic::Status;

    fn try_from(c: ch_ewf_grpc::psc::CorporateEntity) -> Result<Self, Self::Error> {
        if c.corporate_name.is_empty() || c.corporate_name.len() > 160 {
            return Err(tonic::Status::invalid_argument("Invalid corporate name"));
        }

        Ok(proto::psc::PSCCorporateEntity {
            corporate_name: c.corporate_name,
            address: match c.address {
                Some(a) => a.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Address required"))
            },
            company_identification: match c.corporate_identification {
                Some(i) => i.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Corporate identification required"))
            },
        })
    }
}

impl TryFrom<ch_ewf_grpc::psc::LegalPerson> for proto::psc::PSCLegalPerson {
    type Error = tonic::Status;

    fn try_from(c: ch_ewf_grpc::psc::LegalPerson) -> Result<Self, Self::Error> {
        if c.name.is_empty() || c.name.len() > 160 {
            return Err(tonic::Status::invalid_argument("Invalid legal person name"));
        }

        Ok(proto::psc::PSCLegalPerson {
            name: c.name,
            address: match c.address {
                Some(a) => a.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Address required"))
            },
            legal_person_identification: match c.legal_person_identification {
                Some(i) => i.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Legal person identification required"))
            },
        })
    }
}

impl TryFrom<ch_ewf_grpc::psc::Individual> for proto::psc::PSCIndividual<proto::base_types::PersonType2> {
    type Error = tonic::Status;

    fn try_from(c: ch_ewf_grpc::psc::Individual) -> Result<Self, Self::Error> {
        if c.nationality.is_empty() || c.nationality.len() > 50 {
            return Err(tonic::Status::invalid_argument("Invalid nationality"));
        }
        if c.country_of_residence.is_empty() || c.country_of_residence.len() > 50 {
            return Err(tonic::Status::invalid_argument("Invalid country of residence"));
        }
        let mut person = match c.person {
            Some(a) => a,
            None => return Err(tonic::Status::invalid_argument("Name required"))
        };
        if person.surname.is_empty() || person.surname.len() > 160 {
            return Err(tonic::Status::invalid_argument("Invalid surname"));
        }

        Ok(proto::psc::PSCIndividual {
            person: proto::base_types::PersonType2 {
                title: if person.title.is_empty() {
                    None
                } else {
                    if person.title.len() > 50 {
                        return Err(tonic::Status::invalid_argument("Invalid title"));
                    }
                    Some(person.title)
                },
                forename: if person.forenames.is_empty() {
                    None
                } else {
                    let forename = person.forenames.remove(0);
                    if forename.len() > 50 {
                        return Err(tonic::Status::invalid_argument("Invalid forename"));
                    }
                    Some(forename)
                },
                other_forenames: if person.forenames.is_empty() {
                    None
                } else {
                    let other_forename = person.forenames.join(" ");
                    if other_forename.len() > 50 {
                        return Err(tonic::Status::invalid_argument("Invalid forename"));
                    }
                    Some(other_forename)
                },
                surname: person.surname,
            },
            service_address: match c.service_address {
                Some(a) => a.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Service address required"))
            },
            date_of_birth: match proto_to_chrono(c.date_of_birth) {
                Some(a) => a.date(),
                None => return Err(tonic::Status::invalid_argument("Date of birth required"))
            },
            nationality: c.nationality,
            country_of_residence: Some(c.country_of_residence),
            residential_address: match c.residential_address {
                Some(a) => a.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Residential address required"))
            },
        })
    }
}

impl TryFrom<ch_ewf_grpc::psc::CorporateIdentification> for proto::psc::PSCCorporateIdentification {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::psc::CorporateIdentification) -> Result<Self, Self::Error> {
        if value.law_governed.is_empty() || value.law_governed.len() > 160 {
            return Err(tonic::Status::invalid_argument("Invalid law governed"));
        }
        if value.legal_form.is_empty() || value.legal_form.len() > 160 {
            return Err(tonic::Status::invalid_argument("Invalid legal form"));
        }

        Ok(proto::psc::PSCCorporateIdentification {
            place_registered: if value.place_registered.is_empty() {
                None
            } else {
                if value.place_registered.len() > 160 {
                    return Err(tonic::Status::invalid_argument("Invalid place registered"));
                }
                Some(value.place_registered)
            },
            registration_number: if value.registration_number.is_empty() {
                None
            } else {
                if value.registration_number.len() > 160 {
                    return Err(tonic::Status::invalid_argument("Invalid registration number"));
                }
                Some(value.registration_number)
            },
            law_governed: value.law_governed,
            legal_form: value.legal_form,
            country_or_state: if value.country_or_state.is_empty() {
                None
            } else {
                if value.country_or_state.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid country or state"));
                }
                Some(value.country_or_state)
            },
        })
    }
}

impl TryFrom<ch_ewf_grpc::psc::LegalPersonIdentification> for proto::psc::PSCLegalPersonIdentification {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::psc::LegalPersonIdentification) -> Result<Self, Self::Error> {
        if value.law_governed.is_empty() || value.law_governed.len() > 160 {
            return Err(tonic::Status::invalid_argument("Invalid law governed"));
        }
        if value.legal_form.is_empty() || value.legal_form.len() > 160 {
            return Err(tonic::Status::invalid_argument("Invalid legal form"));
        }

        Ok(proto::psc::PSCLegalPersonIdentification {
            law_governed: value.law_governed,
            legal_form: value.legal_form,
        })
    }
}


impl TryFrom<ch_ewf_grpc::psc::IndividualIdentification> for proto::psc::PSCIdentification {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::psc::IndividualIdentification) -> Result<Self, Self::Error> {
        Ok(proto::psc::PSCIdentification {
            name: match value.name {
                Some(n) => n.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Name required"))
            },
            partial_dob: value.partial_dob.map(|d| {
                if d.month < 1 || d.month > 12 {
                    return Err(tonic::Status::invalid_argument("Invalid month"));
                }

                Ok(proto::base_types::PartialDOBType {
                    month: d.month,
                    year: d.year,
                })
            }).transpose()?,
        })
    }
}

impl TryFrom<ch_ewf_grpc::psc::NatureOfControls> for proto::psc::PSCNatureOfControls {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::psc::NatureOfControls) -> Result<Self, Self::Error> {
        match value.nature_of_controls {
            Some(ch_ewf_grpc::psc::nature_of_controls::NatureOfControls::CompanyNatureOfControls(c)) => {
                Ok(proto::psc::PSCNatureOfControls::NatureOfControls(proto::psc::NatureOfControls {
                    nature_of_control: c.nature_of_controls.into_iter().map(|c| Ok(match ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::from_i32(c) {
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares25To50) => proto::psc::NatureOfControlType::OwnershipOfShares25To50,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares50To75) => proto::psc::NatureOfControlType::OwnershipOfShares50To75,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares75To100) => proto::psc::NatureOfControlType::OwnershipOfShares75To100,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares25To50AsTrust) => proto::psc::NatureOfControlType::OwnershipOfShares25To50AsTrust,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares50To75AsTrust) => proto::psc::NatureOfControlType::OwnershipOfShares50To75AsTrust,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares75To100AsTrust) => proto::psc::NatureOfControlType::OwnershipOfShares75To100AsTrust,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares25To50AsFirm) => proto::psc::NatureOfControlType::OwnershipOfShares25To50AsFirm,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares50To75AsFirm) => proto::psc::NatureOfControlType::OwnershipOfShares50To75AsFirm,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares75To100AsFirm) => proto::psc::NatureOfControlType::OwnershipOfShares75To100AsFirm,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights25To50) => proto::psc::NatureOfControlType::VotingRights25To50,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights50To75) => proto::psc::NatureOfControlType::VotingRights50To75,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights75To100) => proto::psc::NatureOfControlType::VotingRights75To100,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights25To50AsTrust) => proto::psc::NatureOfControlType::VotingRights25To50AsTrust,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights50To75AsTrust) => proto::psc::NatureOfControlType::VotingRights50To75AsTrust,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights75To100AsTrust) => proto::psc::NatureOfControlType::VotingRights75To100AsTrust,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights25To50AsFirm) => proto::psc::NatureOfControlType::VotingRights25To50AsFirm,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights50To75AsFirm) => proto::psc::NatureOfControlType::VotingRights50To75AsFirm,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights75To100AsFirm) => proto::psc::NatureOfControlType::VotingRights75To100AsFirm,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::RightToAppointAndRemoveDirectors) => proto::psc::NatureOfControlType::RightToAppointAndRemoveDirectors,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::RightToAppointAndRemoveDirectorsAsTrust) => proto::psc::NatureOfControlType::RightToAppointAndRemoveDirectorsAsTrust,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::RightToAppointAndRemoveDirectorsAsFirm) => proto::psc::NatureOfControlType::RightToAppointAndRemoveDirectorsAsFirm,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::SignificantInfluence) => proto::psc::NatureOfControlType::SignificantInfluence,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::SignificantInfluenceAsTrust) => proto::psc::NatureOfControlType::SignificantInfluenceAsTrust,
                        Some(ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::SignificantInfluenceAsFirm) => proto::psc::NatureOfControlType::SignificantInfluenceAsFirm,
                        None => return Err(tonic::Status::invalid_argument("Invalid nature of control"))
                    })).collect::<Result<Vec<_>, _>>()?
                }))
            }
            Some(ch_ewf_grpc::psc::nature_of_controls::NatureOfControls::LlpNatureOfControls(c)) => {
                Ok(proto::psc::PSCNatureOfControls::LLPNatureOfControls(proto::psc::LLPNatureOfControls {
                    nature_of_control: c.nature_of_controls.into_iter().map(|c| Ok(match ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::from_i32(c) {
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets25To50) => proto::psc::LLPNatureOfControlType::RightToSurplusAssets25To50,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets50To75) => proto::psc::LLPNatureOfControlType::RightToSurplusAssets50To75,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets75To100) => proto::psc::LLPNatureOfControlType::RightToSurplusAssets75To100,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets25To50AsTrust) => proto::psc::LLPNatureOfControlType::RightToSurplusAssets25To50AsTrust,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets50To75AsTrust) => proto::psc::LLPNatureOfControlType::RightToSurplusAssets50To75AsTrust,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets75To100AsTrust) => proto::psc::LLPNatureOfControlType::RightToSurplusAssets75To100AsTrust,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets25To50AsFirm) => proto::psc::LLPNatureOfControlType::RightToSurplusAssets25To50AsFirm,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets50To75AsFirm) => proto::psc::LLPNatureOfControlType::RightToSurplusAssets50To75AsFirm,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets75To100AsFirm) => proto::psc::LLPNatureOfControlType::RightToSurplusAssets75To100AsFirm,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights25To50) => proto::psc::LLPNatureOfControlType::VotingRights25To50,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights50To75) => proto::psc::LLPNatureOfControlType::VotingRights50To75,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights75To100) => proto::psc::LLPNatureOfControlType::VotingRights75To100,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights25To50AsTrust) => proto::psc::LLPNatureOfControlType::VotingRights25To50AsTrust,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights50To75AsTrust) => proto::psc::LLPNatureOfControlType::VotingRights50To75AsTrust,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights75To100AsTrust) => proto::psc::LLPNatureOfControlType::VotingRights75To100AsTrust,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights25To50AsFirm) => proto::psc::LLPNatureOfControlType::VotingRights25To50AsFirm,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights50To75AsFirm) => proto::psc::LLPNatureOfControlType::VotingRights50To75AsFirm,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights75To100AsFirm) => proto::psc::LLPNatureOfControlType::VotingRights75To100AsFirm,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToAppointAndRemoveMembers) => proto::psc::LLPNatureOfControlType::RightToAppointAndRemoveMembers,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToAppointAndRemoveMembersAsTrust) => proto::psc::LLPNatureOfControlType::RightToAppointAndRemoveMembersAsTrust,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToAppointAndRemoveMembersAsFirm) => proto::psc::LLPNatureOfControlType::RightToAppointAndRemoveMembersAsFirm,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::SignificantInfluence) => proto::psc::LLPNatureOfControlType::SignificantInfluence,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::SignificantInfluenceAsTrust) => proto::psc::LLPNatureOfControlType::SignificantInfluenceAsTrust,
                        Some(ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::SignificantInfluenceAsFirm) => proto::psc::LLPNatureOfControlType::SignificantInfluenceAsFirm,
                        None => return Err(tonic::Status::invalid_argument("Invalid nature of control"))
                    })).collect::<Result<Vec<_>, _>>()?
                }))
            }
            None => Err(tonic::Status::invalid_argument("Nature of controls required"))
        }
    }
}

impl TryFrom<ch_ewf_grpc::psc::StatementNotification> for proto::psc::PSCStatementNotificationType {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::psc::StatementNotification) -> Result<Self, Self::Error> {
        match value.notification {
            Some(ch_ewf_grpc::psc::statement_notification::Notification::CompanyLevelStatement(c)) => {
                Ok(proto::psc::PSCStatementNotificationType::Company(match ch_ewf_grpc::psc::CompanyLevelStatement::from_i32(c) {
                    Some(ch_ewf_grpc::psc::CompanyLevelStatement::NoSignificantControl) => proto::psc::CompanyLevelStatement::NoSignificantControl,
                    Some(ch_ewf_grpc::psc::CompanyLevelStatement::StepsNotCompleted) => proto::psc::CompanyLevelStatement::StepsNotCompleted,
                    None => return Err(tonic::Status::invalid_argument("Invalid company level statement"))
                }))
            }
            Some(ch_ewf_grpc::psc::statement_notification::Notification::PscLevelStatement(c)) => {
                Ok(proto::psc::PSCStatementNotificationType::Psc(match ch_ewf_grpc::psc::PscLevelStatement::from_i32(c) {
                    Some(ch_ewf_grpc::psc::PscLevelStatement::ExistsButNotIdentified) => proto::psc::PSCLevelStatement::ExistsButNotIdentified,
                    Some(ch_ewf_grpc::psc::PscLevelStatement::DetailsNotConfirmed) => proto::psc::PSCLevelStatement::DetailsNotConfirmed,
                    Some(ch_ewf_grpc::psc::PscLevelStatement::ContactedButNoResponse) => proto::psc::PSCLevelStatement::ContactedButNoResponse,
                    Some(ch_ewf_grpc::psc::PscLevelStatement::RestrictionNoticeIssued) => proto::psc::PSCLevelStatement::RestrictionNoticeIssued,
                    None => return Err(tonic::Status::invalid_argument("Invalid PSC level statement"))
                }))
            }
            Some(ch_ewf_grpc::psc::statement_notification::Notification::LinkedStatement(c)) => {
                Ok(proto::psc::PSCStatementNotificationType::PscLinked(proto::psc::PSCLinkedStatement {
                    statement: match ch_ewf_grpc::psc::linked_statement::LinkedStatementType::from_i32(c.statement) {
                        Some(ch_ewf_grpc::psc::linked_statement::LinkedStatementType::FailedToConfirmChangedDetails) => proto::psc::PSCLinkedStatementType::FailedToConfirmChangedDetails,
                        None => return Err(tonic::Status::invalid_argument("Invalid linked statement"))
                    },
                    entity: match c.psc {
                        Some(ch_ewf_grpc::psc::linked_statement::Psc::Individual(c)) => {
                            proto::psc::PSCLinkedStatementEntity::Individual(c.try_into()?)
                        }
                        Some(ch_ewf_grpc::psc::linked_statement::Psc::CorporateName(c)) => {
                            if c.is_empty() || c.len() > 160 {
                                return Err(tonic::Status::invalid_argument("Invalid corporate name"));
                            }
                            proto::psc::PSCLinkedStatementEntity::Corporate(proto::psc::LinkedStatementCorporate {
                                corporate_name: c
                            })
                        }
                        Some(ch_ewf_grpc::psc::linked_statement::Psc::LegalPersonName(c)) => {
                            if c.is_empty() || c.len() > 160 {
                                return Err(tonic::Status::invalid_argument("Invalid legal person name"));
                            }
                            proto::psc::PSCLinkedStatementEntity::LegalPerson(proto::psc::LinkedStatementLegalPerson {
                                legal_person_name: c
                            })
                        }
                        Some(ch_ewf_grpc::psc::linked_statement::Psc::SuperSecureIndividual(_)) =>
                            return Err(tonic::Status::invalid_argument("Cannot file for a super secure individual")),
                        None => return Err(tonic::Status::invalid_argument("Linked statement entity required"))
                    },
                }))
            }
            None => Err(tonic::Status::invalid_argument("Statement notification required"))
        }
    }
}