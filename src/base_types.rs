use std::convert::{TryFrom, TryInto};
use super::{ch_ewf_grpc, proto};
use super::grpc::proto_to_chrono;

impl From<proto::base_types::UKAddress> for ch_ewf_grpc::base_types::UkAddress {
    fn from(addr: proto::base_types::UKAddress) -> ch_ewf_grpc::base_types::UkAddress {
        ch_ewf_grpc::base_types::UkAddress {
            premise: addr.premise,
            street: addr.street.unwrap_or_default(),
            thoroughfare: addr.thoroughfare.unwrap_or_default(),
            post_town: addr.post_town,
            county: addr.county.unwrap_or_default(),
            country: match addr.country {
                proto::base_types::UKCountry::England => ch_ewf_grpc::base_types::uk_address::Country::England.into(),
                proto::base_types::UKCountry::Scotland => ch_ewf_grpc::base_types::uk_address::Country::Scotland.into(),
                proto::base_types::UKCountry::Wales => ch_ewf_grpc::base_types::uk_address::Country::Wales.into(),
                proto::base_types::UKCountry::NorthernIreland => ch_ewf_grpc::base_types::uk_address::Country::NorthernIreland.into(),
                proto::base_types::UKCountry::Undefined => ch_ewf_grpc::base_types::uk_address::Country::Undefined.into(),
                proto::base_types::UKCountry::GreatBritain => ch_ewf_grpc::base_types::uk_address::Country::Uk.into(),
            },
            postcode: addr.postcode.unwrap_or_default(),
            care_of_name: addr.care_of_name.unwrap_or_default(),
            po_box: addr.po_box.unwrap_or_default(),
        }
    }
}

impl TryFrom<ch_ewf_grpc::base_types::UkAddress> for proto::base_types::UKAddress {
    type Error = tonic::Status;

    fn try_from(a: ch_ewf_grpc::base_types::UkAddress) -> Result<Self, Self::Error> {
        if a.premise.is_empty() || a.premise.len() > 50 {
            return Err(tonic::Status::invalid_argument("Invalid premise"));
        }
        if a.post_town.is_empty() || a.post_town.len() > 50 {
            return Err(tonic::Status::invalid_argument("Invalid post town"));
        }

        Ok(proto::base_types::UKAddress {
            premise: a.premise,
            street: if a.street.is_empty() {
                None
            } else {
                if a.street.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid street"));
                }
                Some(a.street)
            },
            thoroughfare: if a.thoroughfare.is_empty() {
                None
            } else {
                if a.thoroughfare.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid thoroughfare"));
                }
                Some(a.thoroughfare)
            },
            post_town: a.post_town,
            county: if a.county.is_empty() {
                None
            } else {
                if a.county.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid county"));
                }
                Some(a.county)
            },
            postcode: if a.postcode.is_empty() {
                None
            } else {
                if a.postcode.len() > 15 {
                    return Err(tonic::Status::invalid_argument("Invalid postcode"));
                }
                Some(a.postcode)
            },
            country: match ch_ewf_grpc::base_types::uk_address::Country::from_i32(a.country) {
                Some(ch_ewf_grpc::base_types::uk_address::Country::England) => proto::base_types::UKCountry::England,
                Some(ch_ewf_grpc::base_types::uk_address::Country::Scotland) => proto::base_types::UKCountry::Scotland,
                Some(ch_ewf_grpc::base_types::uk_address::Country::Wales) => proto::base_types::UKCountry::Wales,
                Some(ch_ewf_grpc::base_types::uk_address::Country::NorthernIreland) => proto::base_types::UKCountry::NorthernIreland,
                Some(ch_ewf_grpc::base_types::uk_address::Country::Uk) => proto::base_types::UKCountry::GreatBritain,
                _ => proto::base_types::UKCountry::Undefined,
            },
            care_of_name: if a.care_of_name.is_empty() {
                None
            } else {
                if a.care_of_name.len() > 100 {
                    return Err(tonic::Status::invalid_argument("Invalid care of name"));
                }
                Some(a.care_of_name)
            },
            po_box: if a.po_box.is_empty() {
                None
            } else {
                if a.po_box.len() > 10 {
                    return Err(tonic::Status::invalid_argument("Invalid PO box"));
                }
                Some(a.po_box)
            },
        })
    }
}

impl From<proto::base_types::BaseAddress> for ch_ewf_grpc::base_types::BaseAddress {
    fn from(addr: proto::base_types::BaseAddress) -> ch_ewf_grpc::base_types::BaseAddress {
        ch_ewf_grpc::base_types::BaseAddress {
            premise: addr.premise,
            street: addr.street.unwrap_or_default(),
            thoroughfare: addr.thoroughfare.unwrap_or_default(),
            post_town: addr.post_town,
            county: addr.county.unwrap_or_default(),
            country: match addr.country {
                None => "".to_string(),
                Some(proto::base_types::AddressCountry::Country(c)) => match c {
                    proto::base_types::AddressCountryType::England => "GB-ENG",
                    proto::base_types::AddressCountryType::Scotland => "GB-SCO",
                    proto::base_types::AddressCountryType::Wales => "GB-WLS",
                    proto::base_types::AddressCountryType::NorthernIreland => "GB-NIR",
                    proto::base_types::AddressCountryType::GreatBritain => isocountry::alpha2::ISO_A2_GBR,
                    proto::base_types::AddressCountryType::UnitedStatesOfAmerica => isocountry::alpha2::ISO_A2_USA,
                    proto::base_types::AddressCountryType::Ireland => isocountry::alpha2::ISO_A2_IRL,
                    proto::base_types::AddressCountryType::Germany => isocountry::alpha2::ISO_A2_DEU,
                    proto::base_types::AddressCountryType::France => isocountry::alpha2::ISO_A2_FRA,
                    proto::base_types::AddressCountryType::Italy => isocountry::alpha2::ISO_A2_ITA,
                    proto::base_types::AddressCountryType::Spain => isocountry::alpha2::ISO_A2_ESP,
                    proto::base_types::AddressCountryType::Portugal => isocountry::alpha2::ISO_A2_PRT,
                    proto::base_types::AddressCountryType::Netherlands => isocountry::alpha2::ISO_A2_NLD,
                    proto::base_types::AddressCountryType::Poland => isocountry::alpha2::ISO_A2_POL,
                    proto::base_types::AddressCountryType::Belgium => isocountry::alpha2::ISO_A2_BEL,
                    proto::base_types::AddressCountryType::Norway => isocountry::alpha2::ISO_A2_NOR,
                    proto::base_types::AddressCountryType::Sweden => isocountry::alpha2::ISO_A2_SWE,
                    proto::base_types::AddressCountryType::Denmark => isocountry::alpha2::ISO_A2_DNK,
                    proto::base_types::AddressCountryType::Australia => isocountry::alpha2::ISO_A2_AUS,
                    proto::base_types::AddressCountryType::NewZealand => isocountry::alpha2::ISO_A2_NZL,
                    proto::base_types::AddressCountryType::Canada => isocountry::alpha2::ISO_A2_CAN,
                    proto::base_types::AddressCountryType::SouthAfrica => isocountry::alpha2::ISO_A2_ZAF,
                    proto::base_types::AddressCountryType::Austria => isocountry::alpha2::ISO_A2_AUT,
                    proto::base_types::AddressCountryType::Croatia => isocountry::alpha2::ISO_A2_HRV,
                    proto::base_types::AddressCountryType::Cyprus => isocountry::alpha2::ISO_A2_CYP,
                    proto::base_types::AddressCountryType::Czechia => isocountry::alpha2::ISO_A2_CZE,
                    proto::base_types::AddressCountryType::Estonia => isocountry::alpha2::ISO_A2_EST,
                    proto::base_types::AddressCountryType::Hungary => isocountry::alpha2::ISO_A2_HUN,
                    proto::base_types::AddressCountryType::Greece => isocountry::alpha2::ISO_A2_GRC,
                    proto::base_types::AddressCountryType::Lithuania => isocountry::alpha2::ISO_A2_LTU,
                }.to_string(),
                Some(proto::base_types::AddressCountry::OtherCountry(c)) => match
                isocountry::CountryCode::for_alpha3_caseless(&c).ok()
                    .or_else(|| isocountry::CountryCode::for_alpha2_caseless(&c).ok())
                    .or_else(|| c.parse::<u32>().ok().and_then(|i| isocountry::CountryCode::for_id(i).ok())) {
                    Some(c) => c.alpha2().to_string(),
                    None => c
                },
            },
            postcode: addr.postcode.unwrap_or_default(),
        }
    }
}

impl TryFrom<ch_ewf_grpc::base_types::BaseAddress> for proto::base_types::BaseAddress {
    type Error = tonic::Status;

    fn try_from(a: ch_ewf_grpc::base_types::BaseAddress) -> Result<Self, Self::Error> {
        if a.premise.is_empty() || a.premise.len() > 50 {
            return Err(tonic::Status::invalid_argument("Invalid premise"));
        }
        if a.post_town.is_empty() || a.post_town.len() > 50 {
            return Err(tonic::Status::invalid_argument("Invalid post town"));
        }

        Ok(proto::base_types::BaseAddress {
            premise: a.premise,
            street: if a.street.is_empty() {
                None
            } else {
                if a.street.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid street"));
                }
                Some(a.street)
            },
            thoroughfare: if a.thoroughfare.is_empty() {
                None
            } else {
                if a.thoroughfare.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid thoroughfare"));
                }
                Some(a.thoroughfare)
            },
            post_town: a.post_town,
            county: if a.county.is_empty() {
                None
            } else {
                if a.county.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid county"));
                }
                Some(a.county)
            },
            postcode: if a.postcode.is_empty() {
                None
            } else {
                if a.postcode.len() > 15 {
                    return Err(tonic::Status::invalid_argument("Invalid postcode"));
                }
                Some(a.postcode)
            },
            country: if a.country.is_empty() {
                None
            } else {
                Some(match a.country.as_str() {
                    "GB-ENG" => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::England),
                    "GB-SCO" => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Scotland),
                    "GB-WLS" => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Wales),
                    "GB-NIR" => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::NorthernIreland),
                    c => match isocountry::CountryCode::for_alpha2_caseless(c) {
                        Ok(c) => match c {
                            isocountry::CountryCode::GBR => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::GreatBritain),
                            isocountry::CountryCode::USA => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::UnitedStatesOfAmerica),
                            isocountry::CountryCode::IRL => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Ireland),
                            isocountry::CountryCode::DEU => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Germany),
                            isocountry::CountryCode::FRA => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::France),
                            isocountry::CountryCode::ITA => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Italy),
                            isocountry::CountryCode::ESP => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Spain),
                            isocountry::CountryCode::PRT => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Portugal),
                            isocountry::CountryCode::NLD => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Netherlands),
                            isocountry::CountryCode::POL => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Poland),
                            isocountry::CountryCode::BEL => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Belgium),
                            isocountry::CountryCode::NOR => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Norway),
                            isocountry::CountryCode::SWE => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Sweden),
                            isocountry::CountryCode::DNK => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Denmark),
                            isocountry::CountryCode::AUS => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Australia),
                            isocountry::CountryCode::NZL => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::NewZealand),
                            isocountry::CountryCode::CAN => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Canada),
                            isocountry::CountryCode::ZAF => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::SouthAfrica),
                            isocountry::CountryCode::AUT => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Austria),
                            isocountry::CountryCode::HRV => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Croatia),
                            isocountry::CountryCode::CYP => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Cyprus),
                            isocountry::CountryCode::CZE => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Czechia),
                            isocountry::CountryCode::EST => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Estonia),
                            isocountry::CountryCode::HUN => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Hungary),
                            isocountry::CountryCode::GRC => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Greece),
                            isocountry::CountryCode::LTU => proto::base_types::AddressCountry::Country(proto::base_types::AddressCountryType::Lithuania),
                            c => proto::base_types::AddressCountry::OtherCountry(c.alpha3().to_string())
                        },
                        Err(e) => return Err(tonic::Status::invalid_argument(format!("Invalid country code: {}", e)))
                    }
                })
            },
        })
    }
}

impl From<proto::base_types::CompanyAddress> for ch_ewf_grpc::base_types::CompanyAddress {
    fn from(addr: proto::base_types::CompanyAddress) -> Self {
        ch_ewf_grpc::base_types::CompanyAddress {
            base_address: Some(addr.base_address.into()),
            care_of_name: addr.care_of_name.unwrap_or_default(),
            po_box: addr.po_box.unwrap_or_default(),
        }
    }
}

impl TryFrom<ch_ewf_grpc::base_types::CompanyAddress> for proto::base_types::CompanyAddress {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::base_types::CompanyAddress) -> Result<Self, Self::Error> {
        Ok(proto::base_types::CompanyAddress {
            base_address: match value.base_address {
                Some(a) => a.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Base address required"))
            },
            care_of_name: if value.care_of_name.is_empty() {
                None
            } else {
                if value.care_of_name.len() > 100 {
                    return Err(tonic::Status::invalid_argument("Invalid care of name"));
                }
                Some(value.care_of_name)
            },
            po_box: if value.po_box.is_empty() {
                None
            } else {
                if value.po_box.len() > 10 {
                    return Err(tonic::Status::invalid_argument("Invalid PO box"));
                }
                Some(value.po_box)
            },
        })
    }
}

impl From<proto::base_types::CorporateOfficerType> for ch_ewf_grpc::base_types::CorporateOfficer {
    fn from(officer: proto::base_types::CorporateOfficerType) -> Self {
        ch_ewf_grpc::base_types::CorporateOfficer {
            corporate_name: officer.corporate_name,
            address: Some(officer.address.into()),
            company_identification: Some(ch_ewf_grpc::base_types::CompanyIdentification {
                company_identification: officer.company_identification.map(|i| match i {
                    proto::base_types::CompanyIdentification::UK {
                        registration_number,
                    } => ch_ewf_grpc::base_types::company_identification::CompanyIdentification::UkRegistrationNumber(registration_number),
                    proto::base_types::CompanyIdentification::NonUK {
                        place_registered,
                        registration_number,
                        governing_law,
                        legal_form,
                    } => ch_ewf_grpc::base_types::company_identification::CompanyIdentification::NonUk(ch_ewf_grpc::base_types::NonUkCompanyIdentification {
                        place_registered: place_registered.unwrap_or_default(),
                        registration_number: registration_number.unwrap_or_default(),
                        governing_law,
                        legal_form,
                    })
                })
            }),
        }
    }
}

impl TryFrom<ch_ewf_grpc::base_types::CorporateOfficer> for proto::base_types::CorporateOfficerType {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::base_types::CorporateOfficer) -> Result<Self, Self::Error> {
        if value.corporate_name.is_empty() || value.corporate_name.len() > 160 {
            return Err(tonic::Status::invalid_argument("Invalid corporate name"));
        }

        Ok(proto::base_types::CorporateOfficerType {
            corporate_name: value.corporate_name,
            address: match value.address {
                Some(a) => a.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Address required"))
            },
            company_identification: Some(match value.company_identification {
                Some(i) => i.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Company identification required"))
            }),
        })
    }
}

impl TryFrom<ch_ewf_grpc::base_types::CompanyIdentification> for proto::base_types::CompanyIdentification {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::base_types::CompanyIdentification) -> Result<Self, Self::Error> {
        Ok(match value.company_identification {
            Some(ch_ewf_grpc::base_types::company_identification::CompanyIdentification::UkRegistrationNumber(n)) => {
                if n.is_empty() || n.len() > 20 {
                    return Err(tonic::Status::invalid_argument("Invalid company registration number"));
                }
                proto::base_types::CompanyIdentification::UK {
                    registration_number: n
                }
            }
            Some(ch_ewf_grpc::base_types::company_identification::CompanyIdentification::NonUk(n)) => {
                if n.governing_law.is_empty() || n.governing_law.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid company governing law"));
                }
                if n.legal_form.is_empty() || n.legal_form.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid company legal form"));
                }
                proto::base_types::CompanyIdentification::NonUK {
                    place_registered: if n.place_registered.is_empty() {
                        None
                    } else {
                        if n.place_registered.len() > 50 {
                            return Err(tonic::Status::invalid_argument("Invalid company place registered"));
                        }
                        Some(n.place_registered)
                    },
                    registration_number: if n.registration_number.is_empty() {
                        None
                    } else {
                        if n.registration_number.len() > 20 {
                            return Err(tonic::Status::invalid_argument("Invalid company registration number"));
                        }
                        Some(n.registration_number)
                    },
                    governing_law: n.governing_law,
                    legal_form: n.legal_form,
                }
            }
            None => return Err(tonic::Status::invalid_argument("Company identification required number"))
        })
    }
}


impl From<proto::base_types::PersonType> for ch_ewf_grpc::base_types::PersonName {
    fn from(person: proto::base_types::PersonType) -> Self {
        ch_ewf_grpc::base_types::PersonName {
            title: person.title.unwrap_or_default(),
            forenames: match (person.forename, person.other_forenames) {
                (Some(f), None) => vec![f],
                (Some(f), Some(of)) => vec![f, of],
                _ => vec![]
            },
            surname: person.surname,
        }
    }
}

impl From<proto::base_types::PersonReturnType> for ch_ewf_grpc::base_types::PersonName {
    fn from(person: proto::base_types::PersonReturnType) -> Self {
        ch_ewf_grpc::base_types::PersonName {
            title: person.title.unwrap_or_default(),
            forenames: person.forenames,
            surname: person.surname,
        }
    }
}

impl TryFrom<ch_ewf_grpc::base_types::PersonName> for proto::base_types::PersonType {
    type Error = tonic::Status;

    fn try_from(mut value: ch_ewf_grpc::base_types::PersonName) -> Result<Self, Self::Error> {
        if value.surname.is_empty() || value.surname.len() > 160 {
            return Err(tonic::Status::invalid_argument("Invalid surname"));
        }

        Ok(proto::base_types::PersonType {
            title: if value.title.is_empty() {
                None
            } else {
                if value.title.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid title"));
                }
                Some(value.title)
            },
            forename: if value.forenames.is_empty() {
                None
            } else {
                let forename = value.forenames.remove(0);
                if forename.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid forename"));
                }
                Some(forename)
            },
            other_forenames: if value.forenames.is_empty() {
                None
            } else {
                let other_forename = value.forenames.join(" ");
                if other_forename.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid forename"));
                }
                Some(other_forename)
            },
            surname: value.surname,
        })
    }
}

impl TryFrom<ch_ewf_grpc::base_types::PersonName> for proto::base_types::PersonType2 {
    type Error = tonic::Status;

    fn try_from(mut value: ch_ewf_grpc::base_types::PersonName) -> Result<Self, Self::Error> {
        if value.surname.is_empty() || value.surname.len() > 160 {
            return Err(tonic::Status::invalid_argument("Invalid surname"));
        }

        Ok(proto::base_types::PersonType2 {
            title: if value.title.is_empty() {
                None
            } else {
                if value.title.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid title"));
                }
                Some(value.title)
            },
            forename: if value.forenames.is_empty() {
                None
            } else {
                let forename = value.forenames.remove(0);
                if forename.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid forename"));
                }
                Some(forename)
            },
            other_forenames: if value.forenames.is_empty() {
                None
            } else {
                let other_forename = value.forenames.join(" ");
                if other_forename.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid forename"));
                }
                Some(other_forename)
            },
            surname: value.surname,
        })
    }
}

impl TryFrom<ch_ewf_grpc::base_types::Person> for proto::base_types::Person {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::base_types::Person) -> Result<Self, Self::Error> {
        if value.forename.is_empty() || value.forename.len() > 50 {
            return Err(tonic::Status::invalid_argument("Invalid forename"));
        }
        if value.surname.is_empty() || value.surname.len() > 50 {
            return Err(tonic::Status::invalid_argument("Invalid surname"));
        }

        Ok(proto::base_types::Person {
            forename: value.forename,
            surname: value.surname,
        })
    }
}

impl TryFrom<ch_ewf_grpc::officer_resignation::Person> for proto::officer_resignation::PersonChange {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::officer_resignation::Person) -> Result<Self, Self::Error> {
        Ok(proto::officer_resignation::PersonChange {
            person: match value.person {
                Some(p) => p.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Person name required"))
            },
            dob: match proto_to_chrono(value.date_of_birth) {
                Some(d) => d.date(),
                None => return Err(tonic::Status::invalid_argument("Date of birth required"))
            },
        })
    }
}

impl From<proto::base_types::ServiceAddressType> for ch_ewf_grpc::base_types::ServiceAddress {
    fn from(addr: proto::base_types::ServiceAddressType) -> Self {
        ch_ewf_grpc::base_types::ServiceAddress {
            value: if let Some(address) = addr.address {
                Some(ch_ewf_grpc::base_types::service_address::Value::Address((*address).into()))
            } else {
                Some(ch_ewf_grpc::base_types::service_address::Value::SameAsRegisteredOffice(addr.same_as_registered_office))
            }
        }
    }
}

impl TryFrom<ch_ewf_grpc::base_types::ServiceAddress> for proto::base_types::ServiceAddressType {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::base_types::ServiceAddress) -> Result<Self, Self::Error> {
        match value.value {
            Some(ch_ewf_grpc::base_types::service_address::Value::SameAsRegisteredOffice(s)) => {
                if !s {
                    return Err(tonic::Status::invalid_argument("Same as registered office must be true"));
                }
                Ok(proto::base_types::ServiceAddressType {
                    same_as_registered_office: true,
                    address: None
                })
            }
            Some(ch_ewf_grpc::base_types::service_address::Value::Address(a)) => {
                Ok(proto::base_types::ServiceAddressType {
                    same_as_registered_office: false,
                    address: Some(Box::new(a.try_into()?))
                })
            }
            None => Err(tonic::Status::invalid_argument("Service address value required"))
        }
    }
}

impl From<proto::base_types::ResidentialBaseAddress> for ch_ewf_grpc::base_types::residential_address::ResidentialAddress {
    fn from(addr: proto::base_types::ResidentialBaseAddress) -> Self {
        ch_ewf_grpc::base_types::residential_address::ResidentialAddress {
            address: Some(addr.address.into()),
            secure_address: addr.secure_address,
        }
    }
}

impl From<proto::base_types::ResidentialAddressType> for ch_ewf_grpc::base_types::ResidentialAddress {
    fn from(addr: proto::base_types::ResidentialAddressType) -> Self {
        ch_ewf_grpc::base_types::ResidentialAddress {
            value: if let Some(address) = addr.address {
                Some(ch_ewf_grpc::base_types::residential_address::Value::Address(address.into()))
            } else {
                Some(ch_ewf_grpc::base_types::residential_address::Value::SameAsServiceAddress(addr.same_as_service_address))
            }
        }
    }
}

impl TryFrom<ch_ewf_grpc::base_types::ResidentialAddress> for proto::base_types::ResidentialAddressType {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::base_types::ResidentialAddress) -> Result<Self, Self::Error> {
        match value.value {
            Some(ch_ewf_grpc::base_types::residential_address::Value::SameAsServiceAddress(s)) => {
                if !s {
                    return Err(tonic::Status::invalid_argument("Same as service address must be true"));
                }
                Ok(proto::base_types::ResidentialAddressType {
                    same_as_service_address: true,
                    address: None
                })
            }
            Some(ch_ewf_grpc::base_types::residential_address::Value::Address(a)) => {
                Ok(proto::base_types::ResidentialAddressType {
                    same_as_service_address: false,
                    address: Some(proto::base_types::ResidentialBaseAddress {
                        address: match a.address {
                            Some(a) => a.try_into()?,
                            None => return Err(tonic::Status::invalid_argument("Base address required"))
                        },
                        secure_address: a.secure_address,
                    })
                })
            }
            None => Err(tonic::Status::invalid_argument("Residential address value required"))
        }
    }
}

impl From<proto::base_types::PreviousNameType> for ch_ewf_grpc::base_types::PreviousName {
    fn from(n: proto::base_types::PreviousNameType) -> Self {
        ch_ewf_grpc::base_types::PreviousName {
            forename: n.forename.unwrap_or_default(),
            surname: n.surname,
        }
    }
}

impl TryFrom<ch_ewf_grpc::base_types::PreviousName> for proto::base_types::PreviousNameType {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::base_types::PreviousName) -> Result<Self, Self::Error> {
        if value.surname.is_empty() || value.surname.len() > 50 {
            return Err(tonic::Status::invalid_argument("Invalid surname"));
        }

        Ok(proto::base_types::PreviousNameType {
            forename: if value.forename.is_empty() {
                None
            } else {
                if value.forename.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid forename"));
                }
                Some(value.forename)
            },
            surname: value.surname,
        })
    }
}

impl From<proto::base_types::Capital> for ch_ewf_grpc::base_types::Capital {
    fn from(c: proto::base_types::Capital) -> Self {
        ch_ewf_grpc::base_types::Capital {
            currency: c.share_currency,
            total_aggregate_nominal_value: c.total_aggregate_value,
            total_number_of_shares_issued: c.total_issued_shares,
            total_amount_unpaid: c.total_amount_unpaid,
            shares: c.shares.into_iter().map(|s| ch_ewf_grpc::base_types::Share {
                share_class: s.share_class,
                num_shares: s.num_shares,
                aggregate_nominal_value: s.aggregate_value,
                prescribed_particulars: s.particulars,
            }).collect(),
        }
    }
}

impl TryFrom<ch_ewf_grpc::base_types::Capital> for proto::base_types::Capital {
    type Error = tonic::Status;

    fn try_from(c: ch_ewf_grpc::base_types::Capital) -> Result<Self, Self::Error> {
        if c.currency.len() != 3 {
            return Err(tonic::Status::invalid_argument("Invalid currency code"));
        }

        if c.total_aggregate_nominal_value < 0.0 || c.total_aggregate_nominal_value > 99999999999999999999.999999 {
            return Err(tonic::Status::invalid_argument("Invalid total aggregate nominal value"));
        }

        if c.total_number_of_shares_issued < 0.0 || c.total_number_of_shares_issued > 99999999999999999999.999999 {
            return Err(tonic::Status::invalid_argument("Invalid number of shares issued"));
        }

        if c.total_amount_unpaid < 0.0 || c.total_amount_unpaid > 99999999999999999999.999999 {
            return Err(tonic::Status::invalid_argument("Invalid total amount unpaid"));
        }

        Ok(proto::base_types::Capital {
            share_currency: c.currency,
            total_aggregate_value: c.total_aggregate_nominal_value,
            total_issued_shares: c.total_number_of_shares_issued,
            total_amount_unpaid: c.total_amount_unpaid,
            shares: c.shares.into_iter().map(|s| {
                if s.share_class.is_empty() || s.share_class.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid share class"));
                }

                if s.prescribed_particulars.is_empty() || s.prescribed_particulars.len() > 400 {
                    return Err(tonic::Status::invalid_argument("Invalid prescribed particulars"));
                }

                if s.num_shares < 0.0 || s.num_shares > 999999999999999.999999 {
                    return Err(tonic::Status::invalid_argument("Invalid number of shares"));
                }

                if s.aggregate_nominal_value < 0.0 || s.aggregate_nominal_value > 999999999999999.999999 {
                    return Err(tonic::Status::invalid_argument("Invalid aggregate nominal value"));
                }

                Ok(proto::base_types::Share {
                    share_class: s.share_class,
                    num_shares: s.num_shares,
                    aggregate_value: s.aggregate_nominal_value,
                    particulars: s.prescribed_particulars,
                })
            }).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<ch_ewf_grpc::base_types::PersonalAttribute> for proto::base_types::PersonalAttribute {
    type Error = tonic::Status;

    fn try_from(a: ch_ewf_grpc::base_types::PersonalAttribute) -> Result<Self, Self::Error> {
        if a.personal_data.len() != 3 {
            return Err(tonic::Status::invalid_argument("Invalid personal data length".to_string()));
        }

        Ok(proto::base_types::PersonalAttribute {
            personal_attribute: match ch_ewf_grpc::base_types::personal_attribute::PersonalAttributeType::from_i32(a.personal_attribute) {
                Some(ch_ewf_grpc::base_types::personal_attribute::PersonalAttributeType::BirthTown) =>
                    proto::base_types::PersonalAttributeType::BirthTown,
                Some(ch_ewf_grpc::base_types::personal_attribute::PersonalAttributeType::Telephone) =>
                    proto::base_types::PersonalAttributeType::Telephone,
                Some(ch_ewf_grpc::base_types::personal_attribute::PersonalAttributeType::NationalInsurance) =>
                    proto::base_types::PersonalAttributeType::NationalInsurance,
                Some(ch_ewf_grpc::base_types::personal_attribute::PersonalAttributeType::PassportNumber) =>
                    proto::base_types::PersonalAttributeType::PassportNumber,
                Some(ch_ewf_grpc::base_types::personal_attribute::PersonalAttributeType::MothersMaiden) =>
                    proto::base_types::PersonalAttributeType::MothersMaiden,
                Some(ch_ewf_grpc::base_types::personal_attribute::PersonalAttributeType::FathersForename) =>
                    proto::base_types::PersonalAttributeType::FathersForename,
                None => return Err(tonic::Status::invalid_argument("Personal attribute type required".to_string()))
            },
            personal_data: a.personal_data,
        })
    }
}