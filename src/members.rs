use std::convert::{TryFrom, TryInto};
use super::{ch_ewf_grpc, proto};
use super::grpc::proto_to_chrono;

impl From<proto::register::MemberName> for ch_ewf_grpc::members_data::MemberName {
    fn from(name: proto::register::MemberName) -> Self {
        ch_ewf_grpc::members_data::MemberName {
            name: Some(match name {
                proto::register::MemberName::Person(p) => ch_ewf_grpc::members_data::member_name::Name::IndividualName(
                    ch_ewf_grpc::members_data::member_name::PersonName {
                        forename: p.forename,
                        surname: p.surname,
                    }
                ),
                proto::register::MemberName::CorporateName(c) =>
                    ch_ewf_grpc::members_data::member_name::Name::CorporateName(c)
            })
        }
    }
}

impl TryFrom<ch_ewf_grpc::members_data::SharesOrStockHeld> for proto::base_types::StocksOrSharesHeld {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::members_data::SharesOrStockHeld) -> Result<Self, Self::Error> {
        Ok(match value.shares_or_stock {
            Some(ch_ewf_grpc::members_data::shares_or_stock_held::SharesOrStock::Shares(s)) => {
                let share = match s.share {
                    Some(s) => s,
                    None => return Err(tonic::Status::invalid_argument("Share required"))
                };

                proto::base_types::StocksOrSharesHeld::SharesHeld(proto::base_types::SharesHeld {
                    shares: share.try_into()?,
                    paid_up: s.amount_paid_up,
                })
            }
            Some(ch_ewf_grpc::members_data::shares_or_stock_held::SharesOrStock::Stock(s)) => {
                if s.stock_class.is_empty() || s.stock_class.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid stock class"));
                }
                if s.amount_held < 0.0 || s.amount_held > 999999999999999.999999 {
                    return Err(tonic::Status::invalid_argument("Invalid amount held"));
                }

                proto::base_types::StocksOrSharesHeld::StocksHeld(proto::base_types::StocksHeld {
                    stock_class: s.stock_class,
                    amount_held: s.amount_held,
                    currency: if s.currency.is_empty() {
                        None
                    } else {
                        if s.currency.len() != 3 {
                            return Err(tonic::Status::invalid_argument("Invalid currency"));
                        }
                        Some(s.currency)
                    },
                })
            }
            None => return Err(tonic::Status::invalid_argument("Shares or stock held required"))
        })
    }
}

impl TryFrom<ch_ewf_grpc::members_data::Share> for proto::base_types::Shares {
    type Error = tonic::Status;

    fn try_from(share: ch_ewf_grpc::members_data::Share) -> Result<Self, Self::Error> {
        if share.num_shares < 0.0 || share.num_shares > 999999999999999.999999 {
            return Err(tonic::Status::invalid_argument("Invalid number of shares"));
        }
        if share.share_class.is_empty() || share.share_class.len() > 50 {
            return Err(tonic::Status::invalid_argument("Invalid share class"));
        }

        Ok(proto::base_types::Shares {
            num_shares: share.num_shares,
            share_class: share.share_class,
            share_reference: if share.share_reference.is_empty() {
                None
            } else {
                if share.share_reference.len() > 100 {
                    return Err(tonic::Status::invalid_argument("Invalid share reference"));
                }
                Some(share.share_reference)
            },
        })
    }
}

impl TryFrom<ch_ewf_grpc::members_data::MemberName> for proto::base_types::CompanyMemberName {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::members_data::MemberName) -> Result<Self, Self::Error> {
        Ok(match value.name {
            Some(ch_ewf_grpc::members_data::member_name::Name::IndividualName(i)) => {
                if i.surname.is_empty() || i.surname.len() > 160 {
                    return Err(tonic::Status::invalid_argument("Invalid surname"));
                }
                if i.forename.is_empty() || i.forename.len() > 160 {
                    return Err(tonic::Status::invalid_argument("Invalid forename"));
                }
                proto::base_types::CompanyMemberName::Person(proto::base_types::Person {
                    forename: i.forename,
                    surname: i.surname,
                })
            }
            Some(ch_ewf_grpc::members_data::member_name::Name::CorporateName(c)) => {
                proto::base_types::CompanyMemberName::CorporateName(c)
            }
            None => return Err(tonic::Status::invalid_argument("Member name required"))
        })
    }
}

impl TryFrom<ch_ewf_grpc::members_data::CompanyMember> for proto::base_types::CompanyMember {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::members_data::CompanyMember) -> Result<Self, Self::Error> {
        Ok(proto::base_types::CompanyMember {
            member_class: if value.class.is_empty() {
                None
            } else {
                if value.class.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid member class"));
                }
                Some(value.class)
            },
            name: match value.name {
                Some(n) => n.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Member name required"))
            },
            address: match value.address {
                Some(n) => n.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Address required"))
            },
        })
    }
}

impl TryFrom<ch_ewf_grpc::members_register_update::CeasedToBeMember> for proto::members_register_update::CeasedToBeMember {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::members_register_update::CeasedToBeMember) -> Result<Self, Self::Error> {
        Ok(proto::members_register_update::CeasedToBeMember {
            date_registered_as_member: match proto_to_chrono(value.date_registered) {
                Some(d) => d.date(),
                None => return Err(tonic::Status::invalid_argument("Date registered required".to_string()))
            },
            ceasation_type: match value.status {
                Some(ch_ewf_grpc::members_register_update::ceased_to_be_member::Status::CeasedToBeMember(c)) => {
                    if !c {
                        return Err(tonic::Status::invalid_argument("Ceased to be member must be true".to_string()));
                    }
                    proto::members_register_update::CeasedToBeMemberType::StateCeasedToBeMember(true)
                }
                Some(ch_ewf_grpc::members_register_update::ceased_to_be_member::Status::DateCeased(d)) => {
                    proto::members_register_update::CeasedToBeMemberType::DateCeasedToBeMember(proto_to_chrono(Some(d)).unwrap().date())
                }
                None => return Err(tonic::Status::invalid_argument("Status required".to_string()))
            },
        })
    }
}