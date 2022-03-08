use std::convert::{TryFrom, TryInto};
use super::{ch_ewf_grpc, proto};
use super::grpc::proto_to_chrono;

impl TryFrom<ch_ewf_grpc::officer_change::ServiceAddress> for proto::officer_change::ServiceAddressChange {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::officer_change::ServiceAddress) -> Result<Self, Self::Error> {
        Ok(proto::officer_change::ServiceAddressChange {
            address: match value.address {
                Some(a) => a.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Service address value required"))
            },
            residential_address_unchanged: value.residential_address_unchanged,
        })
    }
}

impl TryFrom<ch_ewf_grpc::officer_appointment::CorporateOfficerAppointment> for proto::base_types::CorporateOfficerAppointmentType {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::officer_appointment::CorporateOfficerAppointment) -> Result<Self, Self::Error> {
        Ok(proto::base_types::CorporateOfficerAppointmentType {
            name: match value.person {
                Some(n) => Some(n.try_into()?),
                None => None,
            },
            corporate_officer: match value.corporate {
                Some(c) => c.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Corporate officer required"))
            },
        })
    }
}

impl TryFrom<ch_ewf_grpc::officer_change::CorporateChange> for proto::officer_change::CorporateChangeDetails {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::officer_change::CorporateChange) -> Result<Self, Self::Error> {
        Ok(proto::officer_change::CorporateChangeDetails {
            corporate_name: match value.new_corporate_name {
                Some(n) => {
                    if n.is_empty() || n.len() > 160 {
                        return Err(tonic::Status::invalid_argument("Invalid new corporate name"));
                    }
                    Some(n)
                }
                None => None
            },
            address: value.new_address.map(TryInto::try_into).transpose()?,
            company_identification: value.new_company_identification.map(TryInto::try_into).transpose()?,
        })
    }
}


impl TryFrom<ch_ewf_grpc::officer_appointment::Director> for proto::base_types::DirectorAppointmentType {
    type Error = tonic::Status;

    fn try_from(o: ch_ewf_grpc::officer_appointment::Director) -> Result<Self, Self::Error> {
        Ok(match o.director {
            Some(ch_ewf_grpc::officer_appointment::director::Director::Person(p)) => {
                if p.nationality.is_empty() || p.nationality.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid nationality".to_string()));
                }
                if p.occupation.is_empty() || p.occupation.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid occupation".to_string()));
                }

                proto::base_types::DirectorAppointmentType::Person(Box::new(proto::base_types::DirectorPersonType {
                    person: match p.person {
                        Some(a) => a.try_into()?,
                        None => return Err(tonic::Status::invalid_argument("Person name required".to_string()))
                    },
                    service_address: match p.service_address {
                        Some(a) => a.try_into()?,
                        None => return Err(tonic::Status::invalid_argument("Service address required".to_string()))
                    },
                    date_of_birth: match proto_to_chrono(p.date_of_birth) {
                        Some(d) => d.date(),
                        None => return Err(tonic::Status::invalid_argument("Date of birth required".to_string()))
                    },
                    nationality: p.nationality,
                    occupation: p.occupation,
                    country_of_residence: if p.country_of_residence.is_empty() {
                        None
                    } else {
                        if p.country_of_residence.len() > 50 {
                            return Err(tonic::Status::invalid_argument("Invalid country of residence".to_string()));
                        }
                        Some(p.country_of_residence)
                    },
                    previous_names: p.previous_names.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
                    residential_address: match p.residential_address {
                        Some(a) => a.try_into()?,
                        None => return Err(tonic::Status::invalid_argument("Residential address required".to_string()))
                    },
                }))
            }
            Some(ch_ewf_grpc::officer_appointment::director::Director::Corporate(c)) =>
                proto::base_types::DirectorAppointmentType::Corporate(Box::new(c.try_into()?)),
            None => return Err(tonic::Status::invalid_argument("Director type required".to_string()))
        })
    }
}

impl TryFrom<ch_ewf_grpc::officer_appointment::Secretary> for proto::base_types::SecretaryAppointmentType {
    type Error = tonic::Status;

    fn try_from(o: ch_ewf_grpc::officer_appointment::Secretary) -> Result<Self, Self::Error> {
        Ok(match o.secretary {
            Some(ch_ewf_grpc::officer_appointment::secretary::Secretary::Person(p)) =>
                proto::base_types::SecretaryAppointmentType::Person(proto::base_types::SecretaryPersonType {
                    person: match p.person {
                        Some(a) => a.try_into()?,
                        None => return Err(tonic::Status::invalid_argument("Person name required".to_string()))
                    },
                    service_address: match p.service_address {
                        Some(a) => a.try_into()?,
                        None => return Err(tonic::Status::invalid_argument("Service address required".to_string()))
                    },
                    previous_names: p.previous_names.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
                }),
            Some(ch_ewf_grpc::officer_appointment::secretary::Secretary::Corporate(c)) =>
                proto::base_types::SecretaryAppointmentType::Corporate(Box::new(c.try_into()?)),
            None => return Err(tonic::Status::invalid_argument("Secretary type required".to_string()))
        })
    }
}

impl TryFrom<ch_ewf_grpc::officer_appointment::Member> for proto::base_types::MemberAppointmentType {
    type Error = tonic::Status;

    fn try_from(o: ch_ewf_grpc::officer_appointment::Member) -> Result<Self, Self::Error> {
        Ok(proto::base_types::MemberAppointmentType {
            designated: o.designated,
            inner: match o.member {
                Some(ch_ewf_grpc::officer_appointment::member::Member::Person(p)) =>
                    proto::base_types::MemberAppointmentTypeInner::Person(Box::new(proto::base_types::MemberPersonType {
                        person: match p.person {
                            Some(a) => a.try_into()?,
                            None => return Err(tonic::Status::invalid_argument("Person name required".to_string()))
                        },
                        service_address: match p.service_address {
                            Some(a) => a.try_into()?,
                            None => return Err(tonic::Status::invalid_argument("Service address required".to_string()))
                        },
                        date_of_birth: match proto_to_chrono(p.date_of_birth) {
                            Some(d) => d.date(),
                            None => return Err(tonic::Status::invalid_argument("Date of birth required".to_string()))
                        },
                        country_of_residence: if p.country_of_residence.is_empty() {
                            None
                        } else {
                            if p.country_of_residence.len() > 50 {
                                return Err(tonic::Status::invalid_argument("Invalid country of residence".to_string()));
                            }
                            Some(p.country_of_residence)
                        },
                        previous_names: p.previous_names.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
                        residential_address: match p.residential_address {
                            Some(a) => a.try_into()?,
                            None => return Err(tonic::Status::invalid_argument("Residential address required".to_string()))
                        },
                    })),
                Some(ch_ewf_grpc::officer_appointment::member::Member::Corporate(c)) =>
                    proto::base_types::MemberAppointmentTypeInner::Corporate(Box::new(c.try_into()?)),
                None => return Err(tonic::Status::invalid_argument("Member type required".to_string()))
            },
        })
    }
}