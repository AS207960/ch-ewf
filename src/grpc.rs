use super::{proto, gov_talk, ch_ewf_grpc, schema, models};
use std::convert::{TryFrom, TryInto};
use rand::Rng;
use diesel::prelude::*;
use tokio_diesel::{OptionalExtension, AsyncConnection, AsyncRunQueryDsl};

/// Helper function to convert chrono times to protobuf well-known type times
pub fn chrono_to_proto<T: chrono::TimeZone>(
    time: Option<chrono::DateTime<T>>,
) -> Option<prost_types::Timestamp> {
    time.map(|t| prost_types::Timestamp {
        seconds: t.timestamp(),
        nanos: t.timestamp_subsec_nanos() as i32,
    })
}

pub fn proto_to_chrono(
    time: Option<prost_types::Timestamp>,
) -> Option<chrono::DateTime<chrono::Utc>> {
    use chrono::offset::TimeZone;
    match time {
        Some(t) => chrono::Utc
            .timestamp_opt(t.seconds, t.nanos as u32)
            .single(),
        None => None,
    }
}

#[derive(Clone)]
pub struct CHFillingService {
    pub sender: gov_talk::GovTalkSender,
    pub connection: r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>,
    pub documents_path: std::path::PathBuf,
    pub presenter_id: String,
    pub package_reference: String,
}

impl CHFillingService {
    fn map_previous_names(names: Vec<proto::base_types::PreviousNameType>) -> Vec<ch_ewf_grpc::base_types::PreviousName> {
        names.into_iter().map(Into::into).collect()
    }

    fn map_company_type(ct: i32) -> Option<proto::base_types::CompanyType> {
        ch_ewf_grpc::base_types::CompanyType::from_i32(ct).map(|ct| match ct {
            ch_ewf_grpc::base_types::CompanyType::CompanyEnglandAndWales => proto::base_types::CompanyType::EW,
            ch_ewf_grpc::base_types::CompanyType::CompanyScotland => proto::base_types::CompanyType::SC,
            ch_ewf_grpc::base_types::CompanyType::CompanyNorthernIreland => proto::base_types::CompanyType::NI,
            ch_ewf_grpc::base_types::CompanyType::CompanyIreland => proto::base_types::CompanyType::R,
            ch_ewf_grpc::base_types::CompanyType::LimitedLiabilityPartnershipEnglandAndWales => proto::base_types::CompanyType::OC,
            ch_ewf_grpc::base_types::CompanyType::LimitedLiabilityPartnershipScotland => proto::base_types::CompanyType::SO,
            ch_ewf_grpc::base_types::CompanyType::LimitedLiabilityPartnershipNorthernIreland => proto::base_types::CompanyType::NC,
        })
    }

    fn map_record_type(ct: i32) -> Option<proto::base_types::RecordType> {
        ch_ewf_grpc::base_types::RecordType::from_i32(ct).map(|ct| match ct {
            ch_ewf_grpc::base_types::RecordType::Directors => proto::base_types::RecordType::Directors,
            ch_ewf_grpc::base_types::RecordType::DirectorsServiceContracts => proto::base_types::RecordType::DirectorsServiceContracts,
            ch_ewf_grpc::base_types::RecordType::DirectorsIndemnities => proto::base_types::RecordType::DirectorsIndemnities,
            ch_ewf_grpc::base_types::RecordType::Members => proto::base_types::RecordType::Members,
            ch_ewf_grpc::base_types::RecordType::Secretaries => proto::base_types::RecordType::Secretaries,
            ch_ewf_grpc::base_types::RecordType::RegisterOfInterests => proto::base_types::RecordType::RegisterOfInterests,
            ch_ewf_grpc::base_types::RecordType::PersonsOfSignificantControl => proto::base_types::RecordType::PersonsOfSignificantControl,
            ch_ewf_grpc::base_types::RecordType::InvestigationReports => proto::base_types::RecordType::InvestigationReports,
            ch_ewf_grpc::base_types::RecordType::OwnShareCapital => proto::base_types::RecordType::OwnShareCapital,
            ch_ewf_grpc::base_types::RecordType::OwnSharePurchaseContracts => proto::base_types::RecordType::OwnSharePurchaseContracts,
            ch_ewf_grpc::base_types::RecordType::RegisterOfChargesEnglandWalesAndNorthernIreland => proto::base_types::RecordType::RegisterOfChargesEnglandWalesAndNorthernIreland,
            ch_ewf_grpc::base_types::RecordType::RegisterOfChargesScotland => proto::base_types::RecordType::RegisterOfChargesScotland,
            ch_ewf_grpc::base_types::RecordType::DebentureHolders => proto::base_types::RecordType::DebentureHolders,
            ch_ewf_grpc::base_types::RecordType::ResolutionsAndMeetings => proto::base_types::RecordType::ResolutionsAndMeetings,
            ch_ewf_grpc::base_types::RecordType::LlpMembers => proto::base_types::RecordType::LLPMembers
        })
    }

    fn map_register_type(ct: i32) -> Option<proto::base_types::RegisterType> {
        ch_ewf_grpc::base_types::Register::from_i32(ct).map(|ct| match ct {
            ch_ewf_grpc::base_types::Register::Directors => proto::base_types::RegisterType::Directors,
            ch_ewf_grpc::base_types::Register::DirectorsUsualResidentialAddress => proto::base_types::RegisterType::DirectorsUsualResidentialAddress,
            ch_ewf_grpc::base_types::Register::Secretaries => proto::base_types::RegisterType::Secretaries,
            ch_ewf_grpc::base_types::Register::Members => proto::base_types::RegisterType::Members,
            ch_ewf_grpc::base_types::Register::PersonsOfSignificantControl => proto::base_types::RegisterType::PersonsOfSignificantControl,
            ch_ewf_grpc::base_types::Register::LlpMembers => proto::base_types::RegisterType::LLPMembers,
            ch_ewf_grpc::base_types::Register::LlpMembersUsualResidentialAddress => proto::base_types::RegisterType::LLPMembersUsualResidentialAddress,
        })
    }

    fn check_authentication_code(code: &str) -> Result<(), tonic::Status> {
        if code.len() < 6 || code.len() > 8 {
            return Err(tonic::Status::invalid_argument("Company authentication code of the wrong length"));
        }
        Ok(())
    }

    fn gen_submission_number(conn: &diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>) -> Result<String, tonic::Status> {
        let mut rng = rand::thread_rng();
        loop {
            let submission_number = std::iter::repeat(())
                .map(|()| rng.sample(rand::distributions::Alphanumeric))
                .map(char::from)
                .take(6)
                .collect();
            let submission_count: i64 = match schema::submissions::dsl::submissions
                .filter(schema::submissions::dsl::ch_submission_id.eq(&submission_number))
                .filter(schema::submissions::dsl::received_timestamp.ge((chrono::Utc::now() - chrono::Duration::days(30)).naive_utc()))
                .count()
                .get_result(conn) {
                Ok(c) => c,
                Err(err) => return Err(tonic::Status::internal(format!("Unable to access DB: {}", err)))
            };

            if submission_count == 0 {
                break Ok(submission_number);
            }
        }
    }

    async fn form_submission(
        &self,
        form_submission: Option<ch_ewf_grpc::form_submission::FormSubmission>,
        submission_class: &str, form_type: &str,
        form: proto::form_submission::Form,
        documents: Vec<proto::form_submission::Document>,
    ) -> Result<ch_ewf_grpc::form_submission::SubmissionResponse, tonic::Status> {
        let conn = match self.connection.get() {
            Ok(c) => c,
            Err(err) => return Err(tonic::Status::internal(format!("Unable to get DB connection: {}", err)))
        };

        let submission_number = Self::gen_submission_number(&conn)?;
        let submission_id = uuid::Uuid::new_v4();

        let form_submission = match form_submission {
            Some(f) => f,
            None => return Err(tonic::Status::invalid_argument("Form submission required".to_string()))
        };

        if form_submission.company_name.len() < 3 || form_submission.company_name.len() > 160 {
            return Err(tonic::Status::invalid_argument("Invalid company name length".to_string()));
        }
        Self::check_authentication_code(&form_submission.authentication_code)?;

        let company_type = match Self::map_company_type(form_submission.company_type) {
            Some(c) => c,
            None => return Err(tonic::Status::invalid_argument("Invalid company type".to_string()))
        };

        let contact_details = form_submission.contact_name.is_empty() && form_submission.contact_number.is_empty();
        let res = match gov_talk::exec_govtalk_transaction(
            &self.sender, submission_class,
            proto::govtalk::GovTalkBody::FormSubmission(Box::new(proto::form_submission::FormSubmission {
                form_header: proto::form_submission::FormHeader {
                    company_number: Some(form_submission.company_number),
                    company_type: Some(company_type.clone()),
                    company_name: form_submission.company_name.to_uppercase(),
                    company_authentication_code: Some(form_submission.authentication_code),
                    package_reference: self.package_reference.clone(),
                    language: match ch_ewf_grpc::form_submission::Language::from_i32(form_submission.language) {
                        Some(ch_ewf_grpc::form_submission::Language::English) => proto::form_submission::SubmissionLanguage::English,
                        Some(ch_ewf_grpc::form_submission::Language::Welsh) => proto::form_submission::SubmissionLanguage::Welsh,
                        None => return Err(tonic::Status::invalid_argument("Language required".to_string()))
                    },
                    form_identifier: form_type.to_string(),
                    submission_number: submission_number.clone(),
                    contact_name: if contact_details {
                        None
                    } else {
                        Some(form_submission.contact_name)
                    },
                    contact_number: if contact_details {
                        None
                    } else {
                        Some(form_submission.contact_number)
                    },
                    customer_reference: form_submission.customer_reference.clone(),
                },
                date_signed: match proto_to_chrono(form_submission.date_signed) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Date signed required".to_string()))
                },
                form,
                additional_information: None,
                documents,
            })),
        ).await {
            Ok(r) => r,
            Err(e) => {
                return Err(tonic::Status::unknown(
                    format!("Transaction ID: {}; error description: {}", e.transaction_id, e.errors.into_iter().map(|e| e.msg).collect::<Vec<_>>().join("; "))
                ));
            }
        };

        let new_submission = models::Submission {
            id: submission_id,
            ch_submission_id: submission_number.clone(),
            company_number: Some(format!("{}{}", company_type.to_string(), form_submission.company_number)),
            received_timestamp: res.gateway_timestamp.naive_utc(),
            customer_reference: form_submission.customer_reference,
            status: schema::Status::Pending,
            reject_reference: None,
            examiner_telephone: None,
            examiner_comment: None,
            document_id: None,
            authentication_code: None,
            charge_code: None,
            incorporation_date: None,
        };

        if let Err(err) = diesel::insert_into(schema::submissions::table)
            .values(new_submission)
            .execute(&conn) {
            return Err(tonic::Status::internal(format!("Unable to save submission to DB: {}", err)));
        }

        Ok(ch_ewf_grpc::form_submission::SubmissionResponse {
            transaction_id: res.transaction_id,
            submission_id: submission_id.to_string(),
            ch_submission_number: submission_number,
        })
    }

    pub async fn watcher(&self) {
        'outer: loop {
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;

            let pending_count: i64 = match schema::submissions::dsl::submissions
                .filter(schema::submissions::dsl::status.eq(schema::Status::Pending))
                .count()
                .get_result_async(&self.connection).await {
                Ok(c) => c,
                Err(err) => {
                    error!("Unable to access DB: {}", err);
                    continue;
                }
            };

            if pending_count > 0 {
                let res = match gov_talk::exec_govtalk_transaction(&self.sender, "GetSubmissionStatus", proto::govtalk::GovTalkBody::GetSubmissionStatus(
                    proto::submission_status::GetSubmissionStatus {
                        reference: None,
                        presenter_id: self.presenter_id.clone(),
                    }
                )).await {
                    Ok(c) => c,
                    Err(e) => {
                        if e.errors.iter().all(|e| e.code == 8026) {
                            continue;
                        }
                        error!(
                            "Unable to query submission status: {}, (trans ID: {})",
                            e.errors.into_iter().map(|e| e.msg).collect::<Vec<_>>().join("; "),
                            e.transaction_id
                        );
                        continue;
                    }
                };

                let body = match res.body {
                    Some(proto::govtalk::GovTalkBody::SubmissionStatus(s)) => s,
                    _ => {
                        error!("Mismatched response body received for submission status");
                        continue;
                    }
                };

                for status in body.status {
                    let submission = match schema::submissions::dsl::submissions
                        .filter(schema::submissions::dsl::ch_submission_id.eq(status.submission_number.clone()))
                        .get_result_async::<models::Submission>(&self.connection).await
                        .optional() {
                        Ok(c) => c,
                        Err(err) => {
                            error!("Unable to access DB: {}", err);
                            continue 'outer;
                        }
                    };
                    if let Some(mut submission) = submission {
                        let mut new_rejections = vec![];
                        submission.status = match status.status_code {
                            proto::submission_status::StatusCode::Pending => schema::Status::Pending,
                            proto::submission_status::StatusCode::Accepted => schema::Status::Accepted,
                            proto::submission_status::StatusCode::Rejected => schema::Status::Rejected,
                            proto::submission_status::StatusCode::Parked => schema::Status::Parked,
                            proto::submission_status::StatusCode::InternalFailure => schema::Status::InternalFailure,
                        };
                        submission.customer_reference = status.customer_reference;
                        if submission.company_number.is_none() {
                            submission.company_number = status.company_number;
                        }
                        if let Some(rejections) = status.rejections {
                            submission.reject_reference = rejections.reject_reference;
                            for rejection in rejections.rejections {
                                let new_rejection = models::SubmissionRejection {
                                    id: uuid::Uuid::new_v4(),
                                    submission_id: submission.id,
                                    code: rejection.reject_code,
                                    description: rejection.description,
                                    instance_number: rejection.instance_number,
                                };
                                new_rejections.push(new_rejection);
                            }
                        }
                        if let Some(examiner) = status.examiner {
                            submission.examiner_telephone = Some(examiner.telephone);
                            submission.examiner_comment = examiner.comment;
                        }
                        match status.details {
                            Some(proto::submission_status::StatusDetails::Incorporation(i)) => {
                                let document_id = match self.get_document(&i.document_request_key).await {
                                    Ok(d) => d,
                                    Err(err) => {
                                        error!("Unable to get document: {}", err);
                                        continue 'outer;
                                    }
                                };
                                submission.document_id = Some(document_id);
                                submission.incorporation_date = Some(i.incorporation_date.naive_utc());
                                submission.authentication_code = Some(i.authentication_code)
                            }
                            Some(proto::submission_status::StatusDetails::ChangeOfName(c)) => {
                                let document_id = match self.get_document(&c.document_request_key).await {
                                    Ok(d) => d,
                                    Err(err) => {
                                        error!("Unable to get document: {}", err);
                                        continue 'outer;
                                    }
                                };
                                submission.document_id = Some(document_id);
                            }
                            Some(proto::submission_status::StatusDetails::Charge(c)) => {
                                let document_id = match self.get_document(&c.document_request_key).await {
                                    Ok(d) => d,
                                    Err(err) => {
                                        error!("Unable to get document: {}", err);
                                        continue 'outer;
                                    }
                                };
                                submission.document_id = Some(document_id);
                                submission.charge_code = Some(c.charge_code)
                            }
                            None => {}
                        }
                        if let Err(err) = self.connection.transaction(|c| {
                            diesel::update(schema::submissions::table)
                                .filter(schema::submissions::dsl::id.eq(submission.id))
                                .set(submission)
                                .execute(c)?;

                            for rejection in new_rejections {
                                diesel::insert_into(schema::submission_rejections::table)
                                    .values(rejection)
                                    .execute(c)?;
                            }

                            Ok(())
                        }).await {
                            error!("Unable to access DB: {}", err);
                            continue;
                        }
                    } else {
                        warn!("Unknown submission ID {}", status.submission_number);
                    }
                }

                match gov_talk::exec_govtalk_transaction(&self.sender, "StatusAck", proto::govtalk::GovTalkBody::GetStatusAck {}).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Unable to ack submission status: {}, (trans ID: {})", e.errors.into_iter().map(|e| e.msg).collect::<Vec<_>>().join("; "), e.transaction_id);
                        continue;
                    }
                };
            }
        }
    }

    async fn get_document(
        &self, document_key: &str,
    ) -> Result<uuid::Uuid, String> {
        let res = match gov_talk::exec_govtalk_transaction(&self.sender, "GetDocument", proto::govtalk::GovTalkBody::GetDocument(
            proto::document::GetDocument {
                document_request_key: document_key.to_string()
            }
        )).await {
            Ok(c) => c,
            Err(e) => {
                return Err(format!("Unable to get document: {}, (trans ID: {})", e.errors.into_iter().map(|e| e.msg).collect::<Vec<_>>().join("; "), e.transaction_id));
            }
        };

        let body = match res.body {
            Some(proto::govtalk::GovTalkBody::Document(s)) => s,
            _ => {
                return Err("Mismatched response body received for document".to_string());
            }
        };

        let filename_id = uuid::Uuid::new_v4();
        let filename_ext = match body.document_data.content_type {
            proto::document::ContentType::Pdf => "pdf"
        };
        let filename = format!("{}.{}", filename_id, filename_ext);
        let file_path = self.documents_path.join(&filename);
        let file_data = match body.document_data.content_encoding {
            proto::document::ContentEncoding::Base64 => match base64::decode(body.document_data.contents.replace('\n', "")) {
                Ok(c) => c,
                Err(err) => return Err(format!("Invalid document encoding: {}", err))
            }
        };

        if let Err(err) = tokio::fs::write(file_path, &file_data).await {
            return Err(format!("Unable to save document: {}", err));
        }

        let document_id = uuid::Uuid::new_v4();
        let new_document = models::Documents {
            id: document_id,
            company_number: body.company_number,
            document_date: body.document_date.unwrap_or_else(chrono::Utc::today).naive_utc(),
            document_type: body.document_type.unwrap_or_default(),
            document_id: body.document_id,
            document_filename: body.document_data.filename.unwrap_or_default(),
            storage_filename: filename,
        };

        if let Err(err) = diesel::insert_into(schema::documents::table)
            .values(new_document)
            .execute_async(&self.connection).await {
            return Err(format!("Unable to access DB: {}", err));
        }

        Ok(document_id)
    }
}

impl From<proto::e_reminders::EReminders> for ch_ewf_grpc::e_reminders::EReminders {
    fn from(body: proto::e_reminders::EReminders) -> Self {
        ch_ewf_grpc::e_reminders::EReminders {
            recipients: body.recipients.into_iter().map(|r| ch_ewf_grpc::e_reminders::EReminderRecipient {
                email_address: r.email,
                activated: r.activated,
            }).collect()
        }
    }
}

impl TryFrom<ch_ewf_grpc::company_incorporation::Authorizer> for proto::company_incorporation::AuthoriserType {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::company_incorporation::Authorizer) -> Result<Self, Self::Error> {
        if value.personal_attributes.len() != 3 {
            return Err(tonic::Status::invalid_argument("Invalid number of personal attributes".to_string()));
        }

        Ok(proto::company_incorporation::AuthoriserType {
            name: match value.name {
                Some(ch_ewf_grpc::company_incorporation::authorizer::Name::Person(p)) =>
                    proto::company_incorporation::IncorporationPersonName::Person(p.try_into()?),
                Some(ch_ewf_grpc::company_incorporation::authorizer::Name::Corporate(c)) => {
                    if c.corporate_name.is_empty() || c.corporate_name.len() > 160 {
                        return Err(tonic::Status::invalid_argument("Invalid corporate name".to_string()));
                    }
                    proto::company_incorporation::IncorporationPersonName::Corporate(proto::company_incorporation::CorporateName {
                        person_name: match c.person {
                            Some(p) => p.try_into()?,
                            None => return Err(tonic::Status::invalid_argument("Person name required".to_string()))
                        },
                        corporate_name: c.corporate_name,
                    })
                }
                None => return Err(tonic::Status::invalid_argument("Authorizer name required".to_string()))
            },
            authentication: value.personal_attributes.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<ch_ewf_grpc::company_incorporation::HmrcAddress> for proto::corporation_tax_information::Address {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::company_incorporation::HmrcAddress) -> Result<Self, Self::Error> {
        if value.address_line_1.is_empty() || value.address_line_1.len() > 27 {
            return Err(tonic::Status::invalid_argument("Invalid address line 1".to_string()));
        }
        if value.address_line_2.is_empty() || value.address_line_2.len() > 27 {
            return Err(tonic::Status::invalid_argument("Invalid address line 2".to_string()));
        }

        Ok(proto::corporation_tax_information::Address {
            address_line_1: value.address_line_1,
            address_line_2: value.address_line_2,
            address_line_3: if value.address_line_3.is_empty() {
                None
            } else {
                if value.address_line_3.len() > 27 {
                    return Err(tonic::Status::invalid_argument("Invalid address line 3".to_string()));
                }
                Some(value.address_line_3)
            },
            address_line_4: if value.address_line_4.is_empty() {
                None
            } else {
                if value.address_line_4.len() > 18 {
                    return Err(tonic::Status::invalid_argument("Invalid address line 4".to_string()));
                }
                Some(value.address_line_4)
            },
            post_code: if value.post_code.is_empty() {
                None
            } else {
                if value.post_code.len() > 15 {
                    return Err(tonic::Status::invalid_argument("Invalid postcode".to_string()));
                }
                Some(value.post_code)
            },
            country: if value.country.is_empty() {
                None
            } else {
                if value.country.len() > 20 {
                    return Err(tonic::Status::invalid_argument("Invalid country".to_string()));
                }
                Some(value.country)
            },
        })
    }
}

impl TryFrom<ch_ewf_grpc::company_incorporation::Person> for proto::company_incorporation::IncorporationPerson {
    type Error = tonic::Status;

    fn try_from(value: ch_ewf_grpc::company_incorporation::Person) -> Result<Self, Self::Error> {
        if value.personal_attributes.len() != 3 {
            return Err(tonic::Status::invalid_argument("Invalid number of personal attributes".to_string()));
        }

        Ok(proto::company_incorporation::IncorporationPerson {
            name: match value.name {
                Some(ch_ewf_grpc::company_incorporation::person::Name::Person(p)) =>
                    proto::company_incorporation::IncorporationPersonName::Person(p.try_into()?),
                Some(ch_ewf_grpc::company_incorporation::person::Name::Corporate(c)) => {
                    if c.corporate_name.is_empty() || c.corporate_name.len() > 160 {
                        return Err(tonic::Status::invalid_argument("Invalid corporate name".to_string()));
                    }
                    proto::company_incorporation::IncorporationPersonName::Corporate(proto::company_incorporation::CorporateName {
                        person_name: match c.person {
                            Some(p) => p.try_into()?,
                            None => return Err(tonic::Status::invalid_argument("Person name required".to_string()))
                        },
                        corporate_name: c.corporate_name,
                    })
                }
                None => return Err(tonic::Status::invalid_argument("Authorizer name required".to_string()))
            },
            address: match value.address {
                Some(a) => a.try_into()?,
                None => return Err(tonic::Status::invalid_argument("Person address required".to_string()))
            },
            authentication: value.personal_attributes.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
            member_class: if value.member_class.is_empty() {
                None
            } else {
                if value.member_class.len() > 50 {
                    return Err(tonic::Status::invalid_argument("Invalid member class".to_string()));
                }
                Some(value.member_class)
            },
        })
    }
}

#[tonic::async_trait]
impl ch_ewf_grpc::ch_filling_server::ChFilling for CHFillingService {
    async fn submission_status(
        &self,
        request: tonic::Request<ch_ewf_grpc::form_submission::SubmissionStatusRequest>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionStatusResponse>, tonic::Status> {
        let msg = request.into_inner();

        let submission_id = match uuid::Uuid::parse_str(&msg.submission_id) {
            Ok(i) => i,
            Err(_) => {
                return Err(tonic::Status::not_found("Invalid submission ID"));
            }
        };
        let submission = match schema::submissions::dsl::submissions
            .filter(schema::submissions::dsl::id.eq(submission_id))
            .get_result_async::<models::Submission>(&self.connection).await
            .optional() {
            Ok(Some(s)) => s,
            Ok(None) => {
                return Err(tonic::Status::not_found("Submission not found"));
            }
            Err(err) => {
                error!("Unable to access DB: {}", err);
                return Err(tonic::Status::internal("Error accessing database"));
            }
        };
        let rejections = match schema::submission_rejections::dsl::submission_rejections
            .filter(schema::submission_rejections::dsl::submission_id.eq(submission.id))
            .get_results_async::<models::SubmissionRejection>(&self.connection).await {
            Ok(r) => r,
            Err(err) => {
                error!("Unable to access DB: {}", err);
                return Err(tonic::Status::internal("Error accessing database"));
            }
        };

        let reply = ch_ewf_grpc::form_submission::SubmissionStatusResponse {
            status: match submission.status {
                schema::Status::Accepted => ch_ewf_grpc::form_submission::SubmissionStatus::Accepted.into(),
                schema::Status::Rejected => ch_ewf_grpc::form_submission::SubmissionStatus::Rejected.into(),
                schema::Status::Pending => ch_ewf_grpc::form_submission::SubmissionStatus::Pending.into(),
                schema::Status::Parked => ch_ewf_grpc::form_submission::SubmissionStatus::Parked.into(),
                schema::Status::InternalFailure => ch_ewf_grpc::form_submission::SubmissionStatus::InternalFailure.into(),
            },
            received_timestamp: chrono_to_proto::<chrono::Utc>(
                Some(chrono::DateTime::from_utc(submission.received_timestamp, chrono::Utc))
            ),
            ch_submission_number: submission.ch_submission_id,
            company_number: submission.company_number.unwrap_or_default(),
            customer_reference: submission.customer_reference.unwrap_or_default(),
            examiner_telephone: submission.examiner_telephone.unwrap_or_default(),
            examiner_comment: submission.examiner_comment.unwrap_or_default(),
            document_id: submission.document_id.map(|d| d.to_string()).unwrap_or_default(),
            charge_code: submission.charge_code.unwrap_or_default(),
            incorporation_date: chrono_to_proto::<chrono::Utc>(
                submission.incorporation_date
                    .map(|d| chrono::DateTime::from_utc(d.and_hms(0, 0, 0), chrono::Utc))
            ),
            authentication_code: submission.authentication_code.unwrap_or_default(),
            reject_reference: submission.reject_reference.unwrap_or_default(),
            rejections: rejections.into_iter().map(|r| ch_ewf_grpc::form_submission::Rejection {
                reject_code: r.code,
                description: r.description,
                instance_number: r.instance_number
            }).collect(),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn document(
        &self,
        request: tonic::Request<ch_ewf_grpc::form_submission::DocumentRequest>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::DocumentResponse>, tonic::Status> {
        let msg = request.into_inner();

        let document_id = match uuid::Uuid::parse_str(&msg.document_id) {
            Ok(i) => i,
            Err(_) => {
                return Err(tonic::Status::not_found("Invalid document ID"));
            }
        };
        let document = match schema::documents::dsl::documents
            .filter(schema::documents::dsl::id.eq(document_id))
            .get_result_async::<models::Documents>(&self.connection).await
            .optional() {
            Ok(Some(s)) => s,
            Ok(None) => {
                return Err(tonic::Status::not_found("Document not found"));
            }
            Err(err) => {
                error!("Unable to access DB: {}", err);
                return Err(tonic::Status::internal("Error accessing database"));
            }
        };

        let file_path = self.documents_path.join(&document.storage_filename);
        let file_data = match tokio::fs::read(file_path).await {
            Ok(d) => d,
            Err(err) => {
                error!("Unable to read document: {}", err);
                return Err(tonic::Status::internal("Error accessing document"));
            }
        };

        let reply = ch_ewf_grpc::form_submission::DocumentResponse {
            date: chrono_to_proto::<chrono::Utc>(Some(
                chrono::DateTime::from_utc(document.document_date.and_hms(0, 0, 0), chrono::Utc)
            )),
            ch_document_id: document.document_id,
            content_type: ch_ewf_grpc::form_submission::ContentType::Pdf.into(),
            ch_filename: document.document_filename,
            data: file_data,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn company_data(
        &self,
        request: tonic::Request<ch_ewf_grpc::company_data::CompanyDataRequest>,
    ) -> Result<tonic::Response<ch_ewf_grpc::company_data::CompanyDataResponse>, tonic::Status> {
        let msg = request.into_inner();
        Self::check_authentication_code(&msg.authentication_code)?;

        let res = match gov_talk::exec_govtalk_transaction(
            &self.sender, "CompanyDataRequest",
            proto::govtalk::GovTalkBody::CompanyDataRequest(proto::company_data::CompanyDataRequest {
                company_number: msg.company_number,
                company_type: Self::map_company_type(msg.company_type),
                company_authentication_code: msg.authentication_code,
                made_up_date: proto_to_chrono(msg.made_up_date).map(|d| d.date()).unwrap_or_else(|| chrono::Utc::now().date()),
            }),
        ).await {
            Ok(r) => r,
            Err(e) => {
                return Err(tonic::Status::unknown(
                    format!("Transaction ID: {}; error description: {}", e.transaction_id, e.errors.into_iter().map(|e| e.msg).collect::<Vec<_>>().join("; "))
                ));
            }
        };

        let body = match res.body {
            Some(proto::govtalk::GovTalkBody::CompanyData(d)) => d,
            _ => {
                return Err(tonic::Status::internal("Mismatched response message received"));
            }
        };

        let reply = ch_ewf_grpc::company_data::CompanyDataResponse {
            transaction_id: res.transaction_id,
            company_number: body.company_number,
            company_name: body.company_name,
            category: match body.company_category {
                proto::company_data::CompanyCategory::Plc => ch_ewf_grpc::company_data::CompanyCategory::Plc.into(),
                proto::company_data::CompanyCategory::ByShares => ch_ewf_grpc::company_data::CompanyCategory::ByShares.into(),
                proto::company_data::CompanyCategory::ByGuarantee => ch_ewf_grpc::company_data::CompanyCategory::ByGuarantee.into(),
                proto::company_data::CompanyCategory::BySharesExemptUnderSection60 => ch_ewf_grpc::company_data::CompanyCategory::BySharesExemptUnderSection60.into(),
                proto::company_data::CompanyCategory::ByGuaranteeExemptUnderSection60 => ch_ewf_grpc::company_data::CompanyCategory::ByGuaranteeExemptUnderSection60.into(),
                proto::company_data::CompanyCategory::UnlimitedWithShareCapital => ch_ewf_grpc::company_data::CompanyCategory::UnlimitedWithShareCapital.into(),
                proto::company_data::CompanyCategory::UnlimitedWithoutShareCapital => ch_ewf_grpc::company_data::CompanyCategory::UnlimitedWithoutShareCapital.into(),
                proto::company_data::CompanyCategory::Llp => ch_ewf_grpc::company_data::CompanyCategory::Llp.into(),
            },
            jurisdiction: match body.jurisdiction {
                proto::company_data::CompanyJurisdiction::EnglandAndWales => ch_ewf_grpc::company_data::Jurisdiction::EnglandAndWales.into(),
                proto::company_data::CompanyJurisdiction::Wales => ch_ewf_grpc::company_data::Jurisdiction::Wales.into(),
                proto::company_data::CompanyJurisdiction::England => ch_ewf_grpc::company_data::Jurisdiction::England.into(),
                proto::company_data::CompanyJurisdiction::Scotland => ch_ewf_grpc::company_data::Jurisdiction::Scotland.into(),
                proto::company_data::CompanyJurisdiction::NorthernIreland => ch_ewf_grpc::company_data::Jurisdiction::NorthernIreland.into(),
                proto::company_data::CompanyJurisdiction::UnitedKingdom => ch_ewf_grpc::company_data::Jurisdiction::Uk.into(),
                proto::company_data::CompanyJurisdiction::EuropeanUnion => ch_ewf_grpc::company_data::Jurisdiction::Eu.into(),
                proto::company_data::CompanyJurisdiction::Other => ch_ewf_grpc::company_data::Jurisdiction::Other.into(),
            },
            trading_on_market: body.trading_on_market,
            dtr5_applies: body.dtr5_applies,
            made_up_date: chrono_to_proto(Some(body.made_up_date.and_hms(0, 0, 0))),
            next_due_date: chrono_to_proto(body.next_due_date.map(|d| d.and_hms(0, 0, 0))),
            psc_exempt_as_shared_admitted_on_market: body.psc_exempt_as_shares_admitted_on_market,
            psc_exempt_as_trading_on_regulated_market: body.psc_exempt_as_trading_on_regulated_market,
            psc_exempt_as_trading_on_uk_regulated_market: body.psc_exempt_as_trading_on_uk_regulated_market,
            sic_codes: body.sic_codes.codes,
            registered_office_address: Some(body.registered_office_address.into()),
            sail_address: body.sail_address.map(Into::into),
            sail_records: body.sail_records.into_iter().map(|r| match r.record_type {
                proto::base_types::RecordType::Directors => ch_ewf_grpc::base_types::RecordType::Directors.into(),
                proto::base_types::RecordType::Members => ch_ewf_grpc::base_types::RecordType::Members.into(),
                proto::base_types::RecordType::DirectorsServiceContracts => ch_ewf_grpc::base_types::RecordType::DirectorsServiceContracts.into(),
                proto::base_types::RecordType::DirectorsIndemnities => ch_ewf_grpc::base_types::RecordType::DirectorsIndemnities.into(),
                proto::base_types::RecordType::Secretaries => ch_ewf_grpc::base_types::RecordType::Secretaries.into(),
                proto::base_types::RecordType::ResolutionsAndMeetings => ch_ewf_grpc::base_types::RecordType::ResolutionsAndMeetings.into(),
                proto::base_types::RecordType::DebentureHolders => ch_ewf_grpc::base_types::RecordType::DebentureHolders.into(),
                proto::base_types::RecordType::RegisterOfChargesEnglandWalesAndNorthernIreland =>
                    ch_ewf_grpc::base_types::RecordType::RegisterOfChargesEnglandWalesAndNorthernIreland.into(),
                proto::base_types::RecordType::RegisterOfChargesScotland => ch_ewf_grpc::base_types::RecordType::RegisterOfChargesScotland.into(),
                proto::base_types::RecordType::OwnSharePurchaseContracts => ch_ewf_grpc::base_types::RecordType::OwnSharePurchaseContracts.into(),
                proto::base_types::RecordType::OwnShareCapital => ch_ewf_grpc::base_types::RecordType::OwnShareCapital.into(),
                proto::base_types::RecordType::InvestigationReports => ch_ewf_grpc::base_types::RecordType::InvestigationReports.into(),
                proto::base_types::RecordType::RegisterOfInterests => ch_ewf_grpc::base_types::RecordType::RegisterOfInterests.into(),
                proto::base_types::RecordType::LLPMembers => ch_ewf_grpc::base_types::RecordType::LlpMembers.into(),
                proto::base_types::RecordType::PersonsOfSignificantControl => ch_ewf_grpc::base_types::RecordType::PersonsOfSignificantControl.into(),
            }).collect(),
            pscs: body.pscs.map(|pscs| match pscs {
                proto::company_data::CompanyDataPSCs::CompanyStatement(s) => {
                    ch_ewf_grpc::company_data::company_data_response::Pscs::PscStatement(match s {
                        proto::psc::CompanyLevelStatement::NoSignificantControl => ch_ewf_grpc::psc::CompanyLevelStatement::NoSignificantControl.into(),
                        proto::psc::CompanyLevelStatement::StepsNotCompleted => ch_ewf_grpc::psc::CompanyLevelStatement::StepsNotCompleted.into(),
                    })
                }
                proto::company_data::CompanyDataPSCs::PSCs(p) => {
                    ch_ewf_grpc::company_data::company_data_response::Pscs::CompanyPscs(ch_ewf_grpc::company_data::CompanyPsCs {
                        pscs: p.into_iter().map(|p| ch_ewf_grpc::company_data::CompanyPsc {
                            psc: Some(match p {
                                proto::company_data::CompanyDataPSC::StatementNotification(n) => {
                                    ch_ewf_grpc::company_data::company_psc::Psc::StatementNotification(match n {
                                        proto::psc::PSCLevelStatement::ExistsButNotIdentified =>
                                            ch_ewf_grpc::psc::PscLevelStatement::ExistsButNotIdentified.into(),
                                        proto::psc::PSCLevelStatement::DetailsNotConfirmed =>
                                            ch_ewf_grpc::psc::PscLevelStatement::DetailsNotConfirmed.into(),
                                        proto::psc::PSCLevelStatement::ContactedButNoResponse =>
                                            ch_ewf_grpc::psc::PscLevelStatement::ContactedButNoResponse.into(),
                                        proto::psc::PSCLevelStatement::RestrictionNoticeIssued =>
                                            ch_ewf_grpc::psc::PscLevelStatement::RestrictionNoticeIssued.into(),
                                    })
                                }
                                proto::company_data::CompanyDataPSC::SuperSecureIndividual(s) => {
                                    ch_ewf_grpc::company_data::company_psc::Psc::SuperSecureIndividual(s)
                                }
                                proto::company_data::CompanyDataPSC::LinkedStatementNotification(l) => {
                                    ch_ewf_grpc::company_data::company_psc::Psc::LinkedStatementNotification(ch_ewf_grpc::psc::LinkedStatement {
                                        statement: match l.statement {
                                            proto::psc::PSCLinkedStatementType::FailedToConfirmChangedDetails =>
                                                ch_ewf_grpc::psc::linked_statement::LinkedStatementType::FailedToConfirmChangedDetails.into()
                                        },
                                        psc: Some(match l.entity {
                                            proto::psc::PSCLinkedStatementEntity::Individual(i) =>
                                                ch_ewf_grpc::psc::linked_statement::Psc::Individual(
                                                    ch_ewf_grpc::psc::IndividualIdentification {
                                                        name: Some(i.name.into()),
                                                        partial_dob: i.partial_dob.map(|d| ch_ewf_grpc::base_types::PartialDob {
                                                            month: d.month,
                                                            year: d.year,
                                                        }),
                                                    }
                                                ),
                                            proto::psc::PSCLinkedStatementEntity::Corporate(n) =>
                                                ch_ewf_grpc::psc::linked_statement::Psc::CorporateName(n.corporate_name),
                                            proto::psc::PSCLinkedStatementEntity::LegalPerson(n) =>
                                                ch_ewf_grpc::psc::linked_statement::Psc::LegalPersonName(n.legal_person_name),
                                            proto::psc::PSCLinkedStatementEntity::SuperSecureIndividual(n) =>
                                                ch_ewf_grpc::psc::linked_statement::Psc::SuperSecureIndividual(n),
                                        }),
                                    })
                                }
                                proto::company_data::CompanyDataPSC::Notification(l) => {
                                    ch_ewf_grpc::company_data::company_psc::Psc::Notification(ch_ewf_grpc::company_data::CompanyDataPscNotification {
                                        notification: Some(ch_ewf_grpc::psc::Notification {
                                            psc: Some(match l.notification {
                                                proto::psc::PSCNotificationType::Corporate(n) =>
                                                    ch_ewf_grpc::psc::notification::Psc::Corporate(
                                                        ch_ewf_grpc::psc::CorporateEntity {
                                                            corporate_name: n.corporate_name,
                                                            address: Some(n.address.into()),
                                                            corporate_identification: Some(ch_ewf_grpc::psc::CorporateIdentification {
                                                                place_registered: n.company_identification.place_registered.unwrap_or_default(),
                                                                registration_number: n.company_identification.registration_number.unwrap_or_default(),
                                                                law_governed: n.company_identification.law_governed,
                                                                legal_form: n.company_identification.legal_form,
                                                                country_or_state: n.company_identification.country_or_state.unwrap_or_default(),
                                                            }),
                                                        }
                                                    ),
                                                proto::psc::PSCNotificationType::LegalPerson(n) =>
                                                    ch_ewf_grpc::psc::notification::Psc::LegalPerson(
                                                        ch_ewf_grpc::psc::LegalPerson {
                                                            name: n.name,
                                                            address: Some(n.address.into()),
                                                            legal_person_identification: Some(ch_ewf_grpc::psc::LegalPersonIdentification {
                                                                legal_form: n.legal_person_identification.legal_form,
                                                                law_governed: n.legal_person_identification.law_governed,
                                                            }),
                                                        }
                                                    ),
                                                proto::psc::PSCNotificationType::Individual(n) => {
                                                    ch_ewf_grpc::psc::notification::Psc::Individual(
                                                        ch_ewf_grpc::psc::Individual {
                                                            person: Some(n.person.into()),
                                                            service_address: Some(n.service_address.to_owned().into()),
                                                            residential_address: Some(n.residential_address.to_owned().into()),
                                                            date_of_birth: chrono_to_proto(Some(n.date_of_birth.and_hms(0, 0, 0))),
                                                            country_of_residence: n.country_of_residence.clone().unwrap_or_default(),
                                                            nationality: n.nationality.clone(),
                                                            consent_statement: false,
                                                        }
                                                    )
                                                }
                                            })
                                        }),
                                        nature_of_controls: Some(ch_ewf_grpc::psc::NatureOfControls {
                                            nature_of_controls: Some(match l.nature_of_control {
                                                proto::psc::PSCNatureOfControls::NatureOfControls(n) =>
                                                    ch_ewf_grpc::psc::nature_of_controls::NatureOfControls::CompanyNatureOfControls(
                                                        ch_ewf_grpc::psc::CompanyNatureOfControls {
                                                            nature_of_controls: n.nature_of_control.into_iter().map(|c| match c {
                                                                proto::psc::NatureOfControlType::OwnershipOfShares25To50 =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares25To50.into(),
                                                                proto::psc::NatureOfControlType::OwnershipOfShares50To75 =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares50To75.into(),
                                                                proto::psc::NatureOfControlType::OwnershipOfShares75To100 =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares75To100.into(),
                                                                proto::psc::NatureOfControlType::OwnershipOfShares25To50AsTrust =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares25To50AsTrust.into(),
                                                                proto::psc::NatureOfControlType::OwnershipOfShares50To75AsTrust =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares50To75AsTrust.into(),
                                                                proto::psc::NatureOfControlType::OwnershipOfShares75To100AsTrust =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares75To100AsTrust.into(),
                                                                proto::psc::NatureOfControlType::OwnershipOfShares25To50AsFirm =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares25To50AsFirm.into(),
                                                                proto::psc::NatureOfControlType::OwnershipOfShares50To75AsFirm =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares50To75AsFirm.into(),
                                                                proto::psc::NatureOfControlType::OwnershipOfShares75To100AsFirm =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::OwnershipOfShares75To100AsFirm.into(),
                                                                proto::psc::NatureOfControlType::VotingRights25To50 =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights25To50.into(),
                                                                proto::psc::NatureOfControlType::VotingRights50To75 =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights50To75.into(),
                                                                proto::psc::NatureOfControlType::VotingRights75To100 =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights75To100.into(),
                                                                proto::psc::NatureOfControlType::VotingRights25To50AsTrust =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights25To50AsTrust.into(),
                                                                proto::psc::NatureOfControlType::VotingRights50To75AsTrust =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights50To75AsTrust.into(),
                                                                proto::psc::NatureOfControlType::VotingRights75To100AsTrust =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights75To100AsTrust.into(),
                                                                proto::psc::NatureOfControlType::VotingRights25To50AsFirm =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights25To50AsFirm.into(),
                                                                proto::psc::NatureOfControlType::VotingRights50To75AsFirm =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights50To75AsFirm.into(),
                                                                proto::psc::NatureOfControlType::VotingRights75To100AsFirm =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::VotingRights75To100AsFirm.into(),
                                                                proto::psc::NatureOfControlType::RightToAppointAndRemoveDirectors =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::RightToAppointAndRemoveDirectors.into(),
                                                                proto::psc::NatureOfControlType::RightToAppointAndRemoveDirectorsAsTrust =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::RightToAppointAndRemoveDirectorsAsTrust.into(),
                                                                proto::psc::NatureOfControlType::RightToAppointAndRemoveDirectorsAsFirm =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::RightToAppointAndRemoveDirectorsAsFirm.into(),
                                                                proto::psc::NatureOfControlType::SignificantInfluence =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::SignificantInfluence.into(),
                                                                proto::psc::NatureOfControlType::SignificantInfluenceAsTrust =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::SignificantInfluenceAsTrust.into(),
                                                                proto::psc::NatureOfControlType::SignificantInfluenceAsFirm =>
                                                                    ch_ewf_grpc::psc::company_nature_of_controls::NatureOfControl::SignificantInfluenceAsFirm.into(),
                                                            }).collect()
                                                        }
                                                    ),
                                                proto::psc::PSCNatureOfControls::LLPNatureOfControls(n) =>
                                                    ch_ewf_grpc::psc::nature_of_controls::NatureOfControls::LlpNatureOfControls(
                                                        ch_ewf_grpc::psc::LlpNatureOfControls {
                                                            nature_of_controls: n.nature_of_control.into_iter().map(|c| match c {
                                                                proto::psc::LLPNatureOfControlType::RightToSurplusAssets25To50 =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets25To50.into(),
                                                                proto::psc::LLPNatureOfControlType::RightToSurplusAssets50To75 =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets25To50.into(),
                                                                proto::psc::LLPNatureOfControlType::RightToSurplusAssets75To100 =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets25To50.into(),
                                                                proto::psc::LLPNatureOfControlType::RightToSurplusAssets25To50AsTrust =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets25To50AsTrust.into(),
                                                                proto::psc::LLPNatureOfControlType::RightToSurplusAssets50To75AsTrust =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets25To50AsTrust.into(),
                                                                proto::psc::LLPNatureOfControlType::RightToSurplusAssets75To100AsTrust =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets25To50AsTrust.into(),
                                                                proto::psc::LLPNatureOfControlType::RightToSurplusAssets25To50AsFirm =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets25To50AsFirm.into(),
                                                                proto::psc::LLPNatureOfControlType::RightToSurplusAssets50To75AsFirm =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets25To50AsFirm.into(),
                                                                proto::psc::LLPNatureOfControlType::RightToSurplusAssets75To100AsFirm =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToSurplusAssets25To50AsFirm.into(),
                                                                proto::psc::LLPNatureOfControlType::VotingRights25To50 =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights25To50.into(),
                                                                proto::psc::LLPNatureOfControlType::VotingRights50To75 =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights25To50.into(),
                                                                proto::psc::LLPNatureOfControlType::VotingRights75To100 =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights25To50.into(),
                                                                proto::psc::LLPNatureOfControlType::VotingRights25To50AsTrust =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights25To50AsTrust.into(),
                                                                proto::psc::LLPNatureOfControlType::VotingRights50To75AsTrust =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights25To50AsTrust.into(),
                                                                proto::psc::LLPNatureOfControlType::VotingRights75To100AsTrust =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights25To50AsTrust.into(),
                                                                proto::psc::LLPNatureOfControlType::VotingRights25To50AsFirm =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights25To50AsFirm.into(),
                                                                proto::psc::LLPNatureOfControlType::VotingRights50To75AsFirm =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights25To50AsFirm.into(),
                                                                proto::psc::LLPNatureOfControlType::VotingRights75To100AsFirm =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::VotingRights25To50AsFirm.into(),
                                                                proto::psc::LLPNatureOfControlType::RightToAppointAndRemoveMembers =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToAppointAndRemoveMembers.into(),
                                                                proto::psc::LLPNatureOfControlType::RightToAppointAndRemoveMembersAsTrust =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToAppointAndRemoveMembersAsTrust.into(),
                                                                proto::psc::LLPNatureOfControlType::RightToAppointAndRemoveMembersAsFirm =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::RightToAppointAndRemoveMembersAsFirm.into(),
                                                                proto::psc::LLPNatureOfControlType::SignificantInfluence =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::SignificantInfluence.into(),
                                                                proto::psc::LLPNatureOfControlType::SignificantInfluenceAsTrust =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::SignificantInfluenceAsTrust.into(),
                                                                proto::psc::LLPNatureOfControlType::SignificantInfluenceAsFirm =>
                                                                    ch_ewf_grpc::psc::llp_nature_of_controls::NatureOfControl::SignificantInfluenceAsFirm.into(),
                                                            }).collect()
                                                        }
                                                    )
                                            })
                                        }),
                                        notification_date: chrono_to_proto(Some(l.notification_date.and_hms(0, 0, 0))),
                                        cessation_date: chrono_to_proto(l.cessation_date.map(|d| d.and_hms(0, 0, 0))),
                                    })
                                }
                            })
                        }).collect()
                    })
                }
            }),
            directors: body.officers.officers.iter()
                .filter_map(|o| match o {
                    proto::company_data::CompanyDataOfficer::Director(d) => Some(d),
                    _ => None
                })
                .map(|o| ch_ewf_grpc::company_data::Director {
                    value: Some(match &o.director_type {
                        proto::company_data::CompanyDataDirectorType::Person(p) =>
                            ch_ewf_grpc::company_data::director::Value::Person(ch_ewf_grpc::base_types::DirectorPerson {
                                person: Some(p.person.to_owned().into()),
                                service_address: Some(p.service_address.to_owned().into()),
                                residential_address: Some(p.residential_address.to_owned().into()),
                                date_of_birth: chrono_to_proto(Some(p.date_of_birth.and_hms(0, 0, 0))),
                                country_of_residence: p.country_of_residence.clone().unwrap_or_default(),
                                nationality: p.nationality.clone(),
                                occupation: p.occupation.clone(),
                                previous_names: Self::map_previous_names(p.previous_names.to_owned()),
                            }),
                        proto::company_data::CompanyDataDirectorType::Corporate(c) =>
                            ch_ewf_grpc::company_data::director::Value::Corporate((*c.to_owned()).into()),
                    }),
                    appointment_date: chrono_to_proto(Some(o.appointment_date.and_hms(0, 0, 0))),
                    resignation_date: chrono_to_proto(o.resignation_date.map(|d| d.and_hms(0, 0, 0))),
                }).collect(),
            secretaries: body.officers.officers.iter()
                .filter_map(|o| match o {
                    proto::company_data::CompanyDataOfficer::Secretary(s) => Some(s),
                    _ => None
                })
                .map(|o| ch_ewf_grpc::company_data::Secretary {
                    value: Some(match &o.secretary_type {
                        proto::company_data::CompanyDataSecretaryType::Person(p) =>
                            ch_ewf_grpc::company_data::secretary::Value::Person(ch_ewf_grpc::base_types::SecretaryPerson {
                                person: Some(p.person.to_owned().into()),
                                service_address: Some(p.service_address.to_owned().into()),
                                previous_names: Self::map_previous_names(p.previous_names.to_owned()),
                            }),
                        proto::company_data::CompanyDataSecretaryType::Corporate(c) =>
                            ch_ewf_grpc::company_data::secretary::Value::Corporate((*c.to_owned()).into()),
                    }),
                    appointment_date: chrono_to_proto(Some(o.appointment_date.and_hms(0, 0, 0))),
                    resignation_date: chrono_to_proto(o.resignation_date.map(|d| d.and_hms(0, 0, 0))),
                }).collect(),
            members: body.officers.officers.iter()
                .filter_map(|o| match o {
                    proto::company_data::CompanyDataOfficer::Member(m) => Some(m),
                    _ => None
                })
                .map(|o| ch_ewf_grpc::company_data::Member {
                    value: Some(match &o.member_type {
                        proto::company_data::CompanyDataMemberType::Person(p) =>
                            ch_ewf_grpc::company_data::member::Value::Person(ch_ewf_grpc::base_types::MemberPerson {
                                person: Some(p.person.to_owned().into()),
                                service_address: Some(p.service_address.clone().into()),
                                residential_address: p.residential_address.to_owned().map(Into::into),
                                date_of_birth: chrono_to_proto(Some(p.date_of_birth.and_hms(0, 0, 0))),
                                country_of_residence: p.country_of_residence.clone().unwrap_or_default(),
                                previous_names: vec![],
                            }),
                        proto::company_data::CompanyDataMemberType::Corporate(c) =>
                            ch_ewf_grpc::company_data::member::Value::Corporate(c.to_owned().into()),
                    }),
                    designated: o.designated,
                }).collect(),
            held_on_public_record: body.registers.map(|s| s.held_on_public_record.into_iter().map(|r| match r.register_type {
                proto::base_types::RegisterType::Directors => ch_ewf_grpc::base_types::Register::Directors.into(),
                proto::base_types::RegisterType::DirectorsUsualResidentialAddress => ch_ewf_grpc::base_types::Register::DirectorsUsualResidentialAddress.into(),
                proto::base_types::RegisterType::Secretaries => ch_ewf_grpc::base_types::Register::Secretaries.into(),
                proto::base_types::RegisterType::Members => ch_ewf_grpc::base_types::Register::Members.into(),
                proto::base_types::RegisterType::LLPMembers => ch_ewf_grpc::base_types::Register::LlpMembers.into(),
                proto::base_types::RegisterType::LLPMembersUsualResidentialAddress => ch_ewf_grpc::base_types::Register::LlpMembersUsualResidentialAddress.into(),
                proto::base_types::RegisterType::PersonsOfSignificantControl => ch_ewf_grpc::base_types::Register::PersonsOfSignificantControl.into(),
            }).collect()).unwrap_or_else(Vec::new),
            statement_of_capital: body.statement_of_captial.map(|s| s.capital.into_iter().map(Into::into).collect()).unwrap_or_else(Vec::new),
            shareholdings: body.shareholdings.into_iter().map(|s| ch_ewf_grpc::company_data::Shareholding {
                share_class: s.share_class,
                number_held: s.num_held,
                shareholders: s.shareholders.into_iter().map(|h| ch_ewf_grpc::base_types::Shareholder {
                    name: Some(match h.name {
                        proto::company_data::CompanyDataShareholderName::Name(n) =>
                            ch_ewf_grpc::base_types::shareholder::Name::PartsName(ch_ewf_grpc::base_types::shareholder::PartsName {
                                forenames: n.forenames,
                                surname: n.surname,
                            }),
                        proto::company_data::CompanyDataShareholderName::AmalgamatedName(n) =>
                            ch_ewf_grpc::base_types::shareholder::Name::AmalgamatedName(n),
                    }),
                    address: Some(h.address.into()),
                }).collect(),
            }).collect(),
        };
        Ok(tonic::Response::new(reply))
    }

    async fn get_e_reminders(
        &self,
        request: tonic::Request<ch_ewf_grpc::e_reminders::GetERemindersRequest>,
    ) -> Result<tonic::Response<ch_ewf_grpc::e_reminders::EReminders>, tonic::Status> {
        let msg = request.into_inner();
        Self::check_authentication_code(&msg.authentication_code)?;

        let res = match gov_talk::exec_govtalk_transaction(
            &self.sender, "GetERemindersRequest",
            proto::govtalk::GovTalkBody::GetERemindersRequest(proto::e_reminders::GetERemindersRequest {
                company_number: msg.company_number,
                company_type: Self::map_company_type(msg.company_type),
                company_authentication_code: msg.authentication_code,
            }),
        ).await {
            Ok(r) => r,
            Err(e) => {
                return Err(tonic::Status::unknown(
                    format!("Transaction ID: {}; error description: {}", e.transaction_id, e.errors.into_iter().map(|e| e.msg).collect::<Vec<_>>().join("; "))
                ));
            }
        };

        let body = match res.body {
            Some(proto::govtalk::GovTalkBody::EReminders(d)) => d,
            _ => {
                return Err(tonic::Status::internal("Mismatched response message received"));
            }
        };

        Ok(tonic::Response::new(body.into()))
    }

    async fn set_e_reminders(
        &self,
        request: tonic::Request<ch_ewf_grpc::e_reminders::SetERemindersRequest>,
    ) -> Result<tonic::Response<ch_ewf_grpc::e_reminders::EReminders>, tonic::Status> {
        let msg = request.into_inner();
        Self::check_authentication_code(&msg.authentication_code)?;

        let res = match gov_talk::exec_govtalk_transaction(
            &self.sender, "SetERemindersRequest",
            proto::govtalk::GovTalkBody::SetERemindersRequest(proto::e_reminders::SetERemindersRequest {
                company_number: msg.company_number,
                company_type: Self::map_company_type(msg.company_type),
                company_authentication_code: msg.authentication_code,
                emails: msg.email_addresses,
            }),
        ).await {
            Ok(r) => r,
            Err(e) => {
                return Err(tonic::Status::unknown(
                    format!("Transaction ID: {}; error description: {}", e.transaction_id, e.errors.into_iter().map(|e| e.msg).collect::<Vec<_>>().join("; "))
                ));
            }
        };

        let body = match res.body {
            Some(proto::govtalk::GovTalkBody::EReminders(d)) => d,
            _ => {
                return Err(tonic::Status::internal("Mismatched response message received"));
            }
        };

        Ok(tonic::Response::new(body.into()))
    }

    async fn payment_periods(
        &self,
        request: tonic::Request<ch_ewf_grpc::payment_periods::PaymentPeriodsRequest>,
    ) -> Result<tonic::Response<ch_ewf_grpc::payment_periods::PaymentPeriodsResponse>, tonic::Status> {
        let msg = request.into_inner();
        Self::check_authentication_code(&msg.authentication_code)?;

        let res = match gov_talk::exec_govtalk_transaction(
            &self.sender, "PaymentPeriodsRequest",
            proto::govtalk::GovTalkBody::PaymentPeriodsRequest(proto::payment_periods::PaymentPeriodsRequest {
                company_number: msg.company_number,
                company_type: Self::map_company_type(msg.company_type),
                company_authentication_code: msg.authentication_code,
            }),
        ).await {
            Ok(r) => r,
            Err(e) => {
                return Err(tonic::Status::unknown(
                    format!("Transaction ID: {}; error description: {}", e.transaction_id, e.errors.into_iter().map(|e| e.msg).collect::<Vec<_>>().join("; "))
                ));
            }
        };

        let body = match res.body {
            Some(proto::govtalk::GovTalkBody::PaymentPeriods(d)) => d,
            _ => {
                return Err(tonic::Status::internal("Mismatched response message received"));
            }
        };

        let reply = ch_ewf_grpc::payment_periods::PaymentPeriodsResponse {
            periods: body.periods.into_iter().map(|p| ch_ewf_grpc::payment_periods::payment_periods_response::PaymentPeriod {
                start_date: chrono_to_proto(Some(p.start_date.and_hms(0, 0, 0))),
                end_date: chrono_to_proto(Some(p.end_date.and_hms(0, 0, 0))),
                period_paid: p.paid,
            }).collect()
        };
        Ok(tonic::Response::new(reply))
    }

    async fn members_register(
        &self,
        request: tonic::Request<ch_ewf_grpc::members_data::MembersRegisterRequest>,
    ) -> Result<tonic::Response<ch_ewf_grpc::members_data::MembersRegisterResponse>, tonic::Status> {
        let msg = request.into_inner();
        Self::check_authentication_code(&msg.authentication_code)?;

        let res = match gov_talk::exec_govtalk_transaction(
            &self.sender, "MembersRegisterDataRequest",
            proto::govtalk::GovTalkBody::MembersDataRequest(proto::members_data::MembersDataRequest {
                company_number: msg.company_number,
                company_type: Self::map_company_type(msg.company_type),
                company_authentication_code: msg.authentication_code,
            }),
        ).await {
            Ok(r) => r,
            Err(e) => {
                return Err(tonic::Status::unknown(
                    format!("Transaction ID: {}; error description: {}", e.transaction_id, e.errors.into_iter().map(|e| e.msg).collect::<Vec<_>>().join("; "))
                ));
            }
        };

        let body = match res.body {
            Some(proto::govtalk::GovTalkBody::MembersData(d)) => d,
            _ => {
                return Err(tonic::Status::internal("Mismatched response message received"));
            }
        };

        let reply = ch_ewf_grpc::members_data::MembersRegisterResponse {
            members: body.members.members.into_iter().map(|m| ch_ewf_grpc::members_data::Member {
                member: Some(match m {
                    proto::members_data::MemberType::MemberWithShares(s) => ch_ewf_grpc::members_data::member::Member::MemberWithShares(
                        ch_ewf_grpc::members_data::MemberWithShares {
                            name: s.name.into_iter().map(Into::into).collect(),
                            shares_or_stock_held: s.stocks_or_shares.into_iter().map(|s| ch_ewf_grpc::members_data::SharesOrStockHeld {
                                shares_or_stock: Some(match s {
                                    proto::register::StocksOrSharesHeld::SharesHeld(s) =>
                                        ch_ewf_grpc::members_data::shares_or_stock_held::SharesOrStock::Shares(
                                            ch_ewf_grpc::members_data::SharesHeld {
                                                share: Some(ch_ewf_grpc::members_data::Share {
                                                    num_shares: s.num_shares,
                                                    share_class: s.share_class,
                                                    share_reference: s.share_reference.unwrap_or_default(),
                                                }),
                                                amount_paid_up: s.paid_up,
                                            }
                                        ),
                                    proto::register::StocksOrSharesHeld::StocksHeld(s) =>
                                        ch_ewf_grpc::members_data::shares_or_stock_held::SharesOrStock::Stock(
                                            ch_ewf_grpc::members_data::StockHeld {
                                                amount_held: s.amount_held,
                                                currency: s.currency,
                                                stock_class: s.stock_class,
                                            }
                                        )
                                })
                            }).collect(),
                            address: Some(s.address.into()),
                            date_registered: chrono_to_proto(Some(s.date_registered.and_hms(0, 0, 0))),
                            date_ceased: chrono_to_proto(s.date_ceased.map(|d| d.and_hms(0, 0, 0))),
                        }
                    ),
                    proto::members_data::MemberType::Member(s) => ch_ewf_grpc::members_data::member::Member::MemberWithoutShares(
                        ch_ewf_grpc::members_data::MemberWithoutShares {
                            member: Some(ch_ewf_grpc::members_data::CompanyMember {
                                class: s.class,
                                name: Some(s.name.into()),
                                address: Some(s.address.into()),
                            }),
                            date_registered: chrono_to_proto(Some(s.date_registered.and_hms(0, 0, 0))),
                            date_ceased: chrono_to_proto(s.date_ceased.map(|d| d.and_hms(0, 0, 0))),
                        }
                    )
                })
            }).collect()
        };
        Ok(tonic::Response::new(reply))
    }

    async fn charge_search(
        &self,
        request: tonic::Request<ch_ewf_grpc::charge_search::ChargeSearchRequest>,
    ) -> Result<tonic::Response<ch_ewf_grpc::charge_search::ChargeSearchResponse>, tonic::Status> {
        let msg = request.into_inner();
        Self::check_authentication_code(&msg.authentication_code)?;

        let mut continuation_key = None;
        let mut charges = vec![];
        loop {
            let res = match gov_talk::exec_govtalk_transaction(
                &self.sender, "ChargeSearch",
                proto::govtalk::GovTalkBody::ChargeSearch(proto::charge_search::ChargeSearch {
                    company_number: msg.company_number,
                    company_type: Self::map_company_type(msg.company_type),
                    company_authentication_code: msg.authentication_code.clone(),
                    start_date: proto_to_chrono(msg.start_date.clone()).map(|d| d.date()),
                    end_date: proto_to_chrono(msg.end_date.clone()).map(|d| d.date()),
                    continuation_key,
                }),
            ).await {
                Ok(r) => r,
                Err(e) => {
                    return Err(tonic::Status::unknown(
                        format!("Transaction ID: {}; error description: {}", e.transaction_id, e.errors.into_iter().map(|e| e.msg).collect::<Vec<_>>().join("; "))
                    ));
                }
            };

            let body = match res.body {
                Some(proto::govtalk::GovTalkBody::Charges(d)) => d,
                _ => {
                    return Err(tonic::Status::internal("Mismatched response message received"));
                }
            };

            continuation_key = body.continuation_key;
            for charge in body.charges {
                charges.push(ch_ewf_grpc::charge_search::Charge {
                    charge_id: Some(match charge.charge_id {
                        proto::charge_search::ChargeID::ChargeCode(c) =>
                            ch_ewf_grpc::charge_search::charge::ChargeId::ChargeCode(c),
                        proto::charge_search::ChargeID::ExistingChargeKey(c) =>
                            ch_ewf_grpc::charge_search::charge::ChargeId::ExistingChargeKey(c)
                    }),
                    created_date: chrono_to_proto(Some(charge.creation_date.and_hms(0, 0, 0))),
                    acquisition_date: chrono_to_proto(charge.acquisition_date.map(|d| d.and_hms(0, 0, 0))),
                    description: Some(match charge.description {
                        proto::charge_search::ChargeDescription::ChargeDescription(d) =>
                            ch_ewf_grpc::charge_search::charge::Description::ChargeDescription(d),
                        proto::charge_search::ChargeDescription::InstrumentDescription(d) =>
                            ch_ewf_grpc::charge_search::charge::Description::InstrumentDescription(ch_ewf_grpc::charge_search::InstrumentDescription {
                                instrument_description: d.instrument_description,
                                short_particulars: d.short_particulars,
                            })
                    }),
                    persons_entitled: charge.persons_entitled,
                    additional_persons_entitled: charge.additional_persons_entitled,
                })
            }

            if continuation_key.is_none() {
                break
            }
        }

        let reply = ch_ewf_grpc::charge_search::ChargeSearchResponse {
            charges
        };
        Ok(tonic::Response::new(reply))
    }

    async fn confirmation_statement(
        &self,
        request: tonic::Request<ch_ewf_grpc::confirmation_statement::ConfirmationStatement>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        if !msg.state_confirmation {
            return Err(tonic::Status::invalid_argument("State confirmation must be true".to_string()));
        }

        let reply = self.form_submission(
            msg.form_submission, "ConfirmationStatement", "ConfirmationStatement",
            proto::form_submission::Form::ConfirmationStatement(proto::confirmation_statement::ConfirmationStatement {
                state_confirmation: msg.state_confirmation,
                review_date: match proto_to_chrono(msg.review_date) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Review date required".to_string()))
                },
                trading_on_market: msg.trading_on_market,
                dtr5_applies: msg.dtr5_applies,
                psc_exempt_as_trading_on_regulated_market: msg.psc_exempt_as_trading_on_regulated_market,
                psc_exempt_as_shares_admitted_on_market: msg.psc_exempt_as_shares_admitted_on_market,
                psc_exempt_as_trading_on_uk_regulated_market: msg.psc_exempt_as_trading_on_uk_regulated_market,
                sic_codes: if msg.sic_codes.is_empty() {
                    None
                } else {
                    Some(proto::base_types::SICCodes {
                        codes: msg.sic_codes.into_iter().map(|sic| {
                            if sic.len() > 5 || sic.len() < 4 || sic.chars().map(|c| c.is_numeric()).any(|x| !x) {
                                Err(tonic::Status::invalid_argument("Invalid SIC code".to_string()))
                            } else {
                                Ok(sic)
                            }
                        }).collect::<Result<Vec<_>, _>>()?
                    })
                },
                statement_of_capital: if msg.statement_of_capital.is_empty() {
                    None
                } else {
                    Some(proto::base_types::StatementOfCapital {
                        capital: msg.statement_of_capital.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?
                    })
                },
                shareholdings: msg.shareholdings.into_iter().map(|s| {
                    if s.share_class.is_empty() || s.share_class.len() > 50 {
                        return Err(tonic::Status::invalid_argument("Invalid share class".to_string()));
                    }

                    if s.number_held < 0.0 || s.number_held > 999999999999999.999999 {
                        return Err(tonic::Status::invalid_argument("Invalid number of shares held".to_string()));
                    }

                    Ok(proto::confirmation_statement::Shareholding {
                        share_class: s.share_class,
                        number_held: s.number_held,
                        transfers: s.transfers.into_iter().map(|t| {
                            if t.number_of_shares_transferred < 0.0 || t.number_of_shares_transferred > 999999999999999.999999 {
                                return Err(tonic::Status::invalid_argument("Invalid number of shares transferred".to_string()));
                            }

                            Ok(proto::confirmation_statement::Transfer {
                                date_of_transfer: match proto_to_chrono(t.date_of_transfer) {
                                    Some(d) => d.date(),
                                    None => return Err(tonic::Status::invalid_argument("Date of transfer required".to_string()))
                                },
                                number_of_shares_transferred: t.number_of_shares_transferred,
                            })
                        }).collect::<Result<Vec<_>, _>>()?,
                        shareholders: s.shareholders.into_iter().map(|h| Ok(proto::confirmation_statement::Shareholder {
                            name: match h.name {
                                Some(ch_ewf_grpc::confirmation_statement::shareholder::Name::PartsName(p)) => {
                                    if p.surname.is_empty() || p.surname.len() > 160 {
                                        return Err(tonic::Status::invalid_argument("Invalid surname".to_string()));
                                    }
                                    if let Some(n) = p.forename.as_ref() {
                                        if n.is_empty() || n.len() > 50 {
                                            return Err(tonic::Status::invalid_argument("Invalid forename".to_string()));
                                        }
                                    }

                                    proto::confirmation_statement::ShareholderName::Name(proto::confirmation_statement::Name {
                                        forename: p.forename,
                                        surname: p.surname,
                                    })
                                }
                                Some(ch_ewf_grpc::confirmation_statement::shareholder::Name::AmalgamatedName(a)) => {
                                    if a.is_empty() || a.len() > 160 {
                                        return Err(tonic::Status::invalid_argument("Invalid amalgamated name".to_string()));
                                    }

                                    proto::confirmation_statement::ShareholderName::AmalgamatedName(a)
                                }
                                None => return Err(tonic::Status::invalid_argument("Shareholder name required".to_string()))
                            },
                            address: match h.address {
                                Some(a) => a.try_into()?,
                                None => return Err(tonic::Status::invalid_argument("Shareholder address required".to_string()))
                            },
                        })).collect::<Result<Vec<_>, _>>()?,
                    })
                }).collect::<Result<Vec<_>, _>>()?,
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn change_registered_office(
        &self,
        request: tonic::Request<ch_ewf_grpc::change_registered_office::ChangeRegisteredOffice>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let reply = self.form_submission(
            msg.form_submission, "ChangeRegisteredOfficeAddress", "ChangeRegisteredOfficeAddress",
            proto::form_submission::Form::ChangeRegisteredOffice(proto::change_registered_office::ChangeRegisteredOfficeAddress {
                address: match msg.address {
                    Some(a) => a.try_into()?,
                    None => return Err(tonic::Status::invalid_argument("Need new address".to_string()))
                }
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn sail_address(
        &self,
        request: tonic::Request<ch_ewf_grpc::sail_address::SailAddress>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let reply = self.form_submission(
            msg.form_submission, "SailAddress", "SailAddress",
            proto::form_submission::Form::SAILAddress(proto::sail_address::SAILAddress {
                address: match msg.address {
                    Some(a) => a.try_into()?,
                    None => return Err(tonic::Status::invalid_argument("Need new address".to_string()))
                }
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn change_of_location(
        &self,
        request: tonic::Request<ch_ewf_grpc::change_of_location::ChangeOfLocation>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let reply = self.form_submission(
            msg.form_submission, "RecordChangeOfLocation", "RecordChangeOfLocation",
            proto::form_submission::Form::RecordChangeOfLocation(match msg.r#move {
                Some(ch_ewf_grpc::change_of_location::change_of_location::Move::MoveToSail(m)) => {
                    if m.records.is_empty() || m.records.len() > 16 {
                        return Err(tonic::Status::invalid_argument("Invalid list of records".to_string()));
                    }
                    proto::change_of_location::RecordChangeOfLocation::MoveToSAILAddress(proto::change_of_location::MoveToSAILAddress {
                        move_to_sail_address: true,
                        register_list: m.records.into_iter().map(|r| Ok(proto::change_of_location::Register {
                            register_type: match Self::map_record_type(r) {
                                Some(r) => r,
                                None => return Err(tonic::Status::invalid_argument("Invalid record type".to_string()))
                            }
                        })).collect::<Result<Vec<_>, _>>()?,
                    })
                }
                Some(ch_ewf_grpc::change_of_location::change_of_location::Move::MoveToRegisteredOffice(m)) => match m.r#move {
                    Some(ch_ewf_grpc::change_of_location::move_to_registered_office::Move::All(b)) => {
                        if !b {
                            return Err(tonic::Status::invalid_argument("Move all to registered office must be true".to_string()));
                        }
                        proto::change_of_location::RecordChangeOfLocation::MoveToRegisteredOffice(proto::change_of_location::MoveToRegisteredOffice {
                            move_to_registered_office: true,
                            move_type: proto::change_of_location::MoveToRegisteredOfficeType::All(true),
                        })
                    }
                    Some(ch_ewf_grpc::change_of_location::move_to_registered_office::Move::Some(r)) => {
                        if r.records.is_empty() || r.records.len() > 16 {
                            return Err(tonic::Status::invalid_argument("Invalid list of records".to_string()));
                        }
                        proto::change_of_location::RecordChangeOfLocation::MoveToRegisteredOffice(proto::change_of_location::MoveToRegisteredOffice {
                            move_to_registered_office: true,
                            move_type: proto::change_of_location::MoveToRegisteredOfficeType::Some(
                                r.records.into_iter().map(|r| Ok(proto::change_of_location::Register {
                                    register_type: match Self::map_record_type(r) {
                                        Some(r) => r,
                                        None => return Err(tonic::Status::invalid_argument("Invalid record type".to_string()))
                                    }
                                })).collect::<Result<Vec<_>, _>>()?,
                            ),
                        })
                    }
                    None => return Err(tonic::Status::invalid_argument("One of move all or move some must be provided".to_string()))
                }
                None => return Err(tonic::Status::invalid_argument("One of move to SAIL or move to RO must be provided".to_string()))
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn officer_appointment(
        &self,
        request: tonic::Request<ch_ewf_grpc::officer_appointment::OfficerAppointment>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        if !msg.consent_to_act {
            return Err(tonic::Status::invalid_argument("Consent to act must be given".to_string()));
        }

        let reply = self.form_submission(
            msg.form_submission, "OfficerAppointment", "OfficerAppointment",
            proto::form_submission::Form::OfficerAppointment(proto::officer_appointment::OfficerAppointment {
                appointment_date: match proto_to_chrono(msg.appointment_date) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Appointment date required".to_string()))
                },
                consent_to_act: true,
                appointment: match msg.appointment {
                    Some(ch_ewf_grpc::officer_appointment::officer_appointment::Appointment::Director(o)) =>
                        proto::officer_appointment::AppointmentType::Director(o.try_into()?),
                    Some(ch_ewf_grpc::officer_appointment::officer_appointment::Appointment::Secretary(o)) =>
                        proto::officer_appointment::AppointmentType::Secretary(o.try_into()?),
                    Some(ch_ewf_grpc::officer_appointment::officer_appointment::Appointment::Member(o)) =>
                        proto::officer_appointment::AppointmentType::Member(Box::new(o.try_into()?)),
                    None => return Err(tonic::Status::invalid_argument("Entity to appoint must be provided".to_string()))
                },
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn officer_resignation(
        &self,
        request: tonic::Request<ch_ewf_grpc::officer_resignation::OfficerResignation>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let reply = self.form_submission(
            msg.form_submission, "OfficerResignation", "OfficerResignation",
            proto::form_submission::Form::OfficerResignation(proto::officer_resignation::OfficerResignation {
                resignation_date: match proto_to_chrono(msg.resignation_date) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Resignation date required".to_string()))
                },
                resignation: match msg.resignation {
                    Some(ch_ewf_grpc::officer_resignation::officer_resignation::Resignation::Director(o)) =>
                        proto::officer_resignation::ResignationType::Director(match o.director {
                            Some(ch_ewf_grpc::officer_resignation::director::Director::Person(p)) =>
                                proto::officer_resignation::DirectorResignation::Person(p.try_into()?),
                            Some(ch_ewf_grpc::officer_resignation::director::Director::Corporate(c)) => {
                                if c.is_empty() || c.len() > 160 {
                                    return Err(tonic::Status::invalid_argument("Invalid corporate name".to_string()));
                                }
                                proto::officer_resignation::DirectorResignation::CorporateName(c)
                            }
                            None => return Err(tonic::Status::invalid_argument("Director type required".to_string()))
                        }),
                    Some(ch_ewf_grpc::officer_resignation::officer_resignation::Resignation::Secretary(o)) =>
                        proto::officer_resignation::ResignationType::Secretary(match o.secretary {
                            Some(ch_ewf_grpc::officer_resignation::secretary::Secretary::Person(p)) =>
                                proto::officer_resignation::SecretaryResignation::Person(p.try_into()?),
                            Some(ch_ewf_grpc::officer_resignation::secretary::Secretary::Corporate(c)) => {
                                if c.is_empty() || c.len() > 160 {
                                    return Err(tonic::Status::invalid_argument("Invalid corporate name".to_string()));
                                }
                                proto::officer_resignation::SecretaryResignation::CorporateName(c)
                            }
                            None => return Err(tonic::Status::invalid_argument("Secretary type required".to_string()))
                        }),
                    Some(ch_ewf_grpc::officer_resignation::officer_resignation::Resignation::Member(o)) =>
                        proto::officer_resignation::ResignationType::Member(match o.member {
                            Some(ch_ewf_grpc::officer_resignation::member::Member::Person(p)) =>
                                proto::officer_resignation::MemberResignation::Person(p.try_into()?),
                            Some(ch_ewf_grpc::officer_resignation::member::Member::Corporate(c)) => {
                                if c.is_empty() || c.len() > 160 {
                                    return Err(tonic::Status::invalid_argument("Invalid corporate name".to_string()));
                                }
                                proto::officer_resignation::MemberResignation::CorporateName(c)
                            }
                            None => return Err(tonic::Status::invalid_argument("Member type required".to_string()))
                        }),
                    None => return Err(tonic::Status::invalid_argument("Entity resigning must be provided".to_string()))
                },
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn officer_change(
        &self,
        request: tonic::Request<ch_ewf_grpc::officer_change::OfficerChange>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let reply = self.form_submission(
            msg.form_submission, "OfficerChangeDetails", "OfficerChangeDetails",
            proto::form_submission::Form::OfficerChangeDetails(proto::officer_change::OfficerChangeDetails {
                date_of_change: match proto_to_chrono(msg.date_of_change) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Date of change required".to_string()))
                },
                change: match msg.change {
                    Some(ch_ewf_grpc::officer_change::officer_change::Change::Director(o)) =>
                        proto::officer_change::ChangeType::Director(match o.director {
                            Some(ch_ewf_grpc::officer_change::director::Director::Person(p)) =>
                                proto::officer_change::DirectorChange::Person(Box::new(proto::officer_change::DirectorPersonChange {
                                    person: match p.person {
                                        Some(p) => p.try_into()?,
                                        None => return Err(tonic::Status::invalid_argument("Person name required".to_string()))
                                    },
                                    dob: match proto_to_chrono(p.date_of_birth) {
                                        Some(d) => d.date(),
                                        None => return Err(tonic::Status::invalid_argument("Date of birth required".to_string()))
                                    },
                                    change: Some(proto::officer_change::DirectorPersonChangeDetails {
                                        person_change: proto::officer_change::PersonChangeDetails {
                                            name: p.new_name.map(TryInto::try_into).transpose()?,
                                            service_address: p.new_service_address.map(TryInto::try_into).transpose()?,
                                        },
                                        residential_address: p.new_residential_address.map(TryInto::try_into).transpose()?,
                                        nationality: match p.new_nationality {
                                            Some(n) => {
                                                if n.is_empty() || n.len() > 50 {
                                                    return Err(tonic::Status::invalid_argument("Invalid nationality".to_string()));
                                                }
                                                Some(n)
                                            }
                                            None => None,
                                        },
                                        country_of_residence: match p.new_country_of_residence {
                                            Some(n) => {
                                                if n.is_empty() || n.len() > 50 {
                                                    return Err(tonic::Status::invalid_argument("Invalid country of residence".to_string()));
                                                }
                                                Some(n)
                                            }
                                            None => None,
                                        },
                                        occupation: match p.new_occupation {
                                            Some(n) => {
                                                if n.is_empty() || n.len() > 50 {
                                                    return Err(tonic::Status::invalid_argument("Invalid occupation".to_string()));
                                                }
                                                Some(n)
                                            }
                                            None => None,
                                        },
                                    }),
                                })),
                            Some(ch_ewf_grpc::officer_change::director::Director::Corporate(c)) => {
                                if c.corporate_name.is_empty() || c.corporate_name.len() > 160 {
                                    return Err(tonic::Status::invalid_argument("Invalid corporate name".to_string()));
                                }

                                proto::officer_change::DirectorChange::Corporate(Box::new(proto::officer_change::CorporateChangeType {
                                    corporate_name: c.corporate_name.clone(),
                                    change: Some(c.try_into()?),
                                }))
                            }
                            None => return Err(tonic::Status::invalid_argument("Director type required".to_string()))
                        }),
                    Some(ch_ewf_grpc::officer_change::officer_change::Change::Secretary(o)) =>
                        proto::officer_change::ChangeType::Secretary(Box::new(match o.secretary {
                            Some(ch_ewf_grpc::officer_change::secretary::Secretary::Person(p)) =>
                                proto::officer_change::SecretaryChange::Person(proto::officer_change::SecretaryPersonChange {
                                    person: match p.person {
                                        Some(p) => p.try_into()?,
                                        None => return Err(tonic::Status::invalid_argument("Person name required".to_string()))
                                    },
                                    change: Some(proto::officer_change::PersonChangeDetails {
                                        name: p.new_name.map(TryInto::try_into).transpose()?,
                                        service_address: match p.new_service_address {
                                            Some(a) => Some(proto::officer_change::ServiceAddressChange {
                                                address: a.try_into()?,
                                                residential_address_unchanged: false,
                                            }),
                                            None => None
                                        },
                                    }),
                                }),
                            Some(ch_ewf_grpc::officer_change::secretary::Secretary::Corporate(c)) => {
                                if c.corporate_name.is_empty() || c.corporate_name.len() > 160 {
                                    return Err(tonic::Status::invalid_argument("Invalid corporate name".to_string()));
                                }

                                proto::officer_change::SecretaryChange::Corporate(proto::officer_change::CorporateChangeType {
                                    corporate_name: c.corporate_name.clone(),
                                    change: Some(c.try_into()?),
                                })
                            }
                            None => return Err(tonic::Status::invalid_argument("Secretary type required".to_string()))
                        })),
                    Some(ch_ewf_grpc::officer_change::officer_change::Change::Member(o)) =>
                        proto::officer_change::ChangeType::Member(match o.member {
                            Some(ch_ewf_grpc::officer_change::member::Member::Person(p)) =>
                                proto::officer_change::MemberChange::Person(Box::new(proto::officer_change::MemberPersonChange {
                                    person: match p.person {
                                        Some(p) => p.try_into()?,
                                        None => return Err(tonic::Status::invalid_argument("Person name required".to_string()))
                                    },
                                    dob: match proto_to_chrono(p.date_of_birth) {
                                        Some(d) => d.date(),
                                        None => return Err(tonic::Status::invalid_argument("Date of birth required".to_string()))
                                    },
                                    change: Some(proto::officer_change::MemberPersonChangeDetails {
                                        person_change: proto::officer_change::PersonChangeDetails {
                                            name: p.new_name.map(TryInto::try_into).transpose()?,
                                            service_address: p.new_service_address.map(TryInto::try_into).transpose()?,
                                        },
                                        residential_address: p.new_residential_address.map(TryInto::try_into).transpose()?,
                                        country_of_residence: match p.new_country_of_residence {
                                            Some(n) => {
                                                if n.is_empty() || n.len() > 50 {
                                                    return Err(tonic::Status::invalid_argument("Invalid nationality".to_string()));
                                                }
                                                Some(n)
                                            }
                                            None => None,
                                        },
                                        designated: p.designated.map(|d| proto::officer_change::MemberDesignated {
                                            designated: d.designated,
                                            consent_to_act: d.consent_to_act,
                                        }),
                                    }),
                                })),
                            Some(ch_ewf_grpc::officer_change::member::Member::Corporate(c)) => {
                                let corporate_change = match c.corporate_change {
                                    Some(c) => c,
                                    None => return Err(tonic::Status::invalid_argument("Corporate change required".to_string()))
                                };

                                if corporate_change.corporate_name.is_empty() || corporate_change.corporate_name.len() > 160 {
                                    return Err(tonic::Status::invalid_argument("Invalid corporate name".to_string()));
                                }

                                proto::officer_change::MemberChange::Corporate(Box::new(proto::officer_change::CorporateMemberChangeType {
                                    corporate_name: corporate_change.corporate_name.clone(),
                                    change: Some(proto::officer_change::CorporateMemberChangeDetails {
                                        change: corporate_change.try_into()?,
                                        designated: c.designated.map(|d| proto::officer_change::MemberDesignated {
                                            designated: d.designated,
                                            consent_to_act: d.consent_to_act,
                                        }),
                                    }),
                                }))
                            }
                            None => return Err(tonic::Status::invalid_argument("Member type required".to_string()))
                        }),
                    None => return Err(tonic::Status::invalid_argument("Entity changing must be provided".to_string()))
                },
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn accounting_reference_date(
        &self,
        request: tonic::Request<ch_ewf_grpc::accounting_reference_date::AccountingReferenceDate>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let reply = self.form_submission(
            msg.form_submission, "ChangeAccountingReferenceDate", "ChangeAccountingReferenceDate",
            proto::form_submission::Form::ChangeAccountingReferenceDate(proto::accounting_reference_date::ChangeAccountingReferenceDate {
                accounting_reference_date: match proto_to_chrono(msg.current_accounting_reference_date) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Current accounting reference date required".to_string()))
                },
                change_to_period: match ch_ewf_grpc::accounting_reference_date::ChangeToPeriod::from_i32(msg.change_to_period) {
                    Some(ch_ewf_grpc::accounting_reference_date::ChangeToPeriod::Shorten) => proto::accounting_reference_date::ChangeToPeriod::Shorten,
                    Some(ch_ewf_grpc::accounting_reference_date::ChangeToPeriod::Extend) => proto::accounting_reference_date::ChangeToPeriod::Extend,
                    None => return Err(tonic::Status::invalid_argument("Invalid change to period".to_string()))
                },
                amended_accounting_reference_date: match proto_to_chrono(msg.new_accounting_reference_date) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("New accounting reference date required".to_string()))
                },
                five_year_extension_details: msg.five_year_extension_details.map(|e| -> Result<_, tonic::Status> {
                    Ok(proto::accounting_reference_date::FiveYearExtensionDetails {
                        extension_reason: match ch_ewf_grpc::accounting_reference_date::ExtensionReason::from_i32(e.extension_reason) {
                            Some(ch_ewf_grpc::accounting_reference_date::ExtensionReason::Administration) => proto::accounting_reference_date::ExtensionReason::Administration,
                            Some(ch_ewf_grpc::accounting_reference_date::ExtensionReason::SecretaryOfState) => proto::accounting_reference_date::ExtensionReason::SecretaryOfState,
                            Some(ch_ewf_grpc::accounting_reference_date::ExtensionReason::UkParent) => proto::accounting_reference_date::ExtensionReason::UKParent,
                            None => return Err(tonic::Status::invalid_argument("Invalid extension reason".to_string()))
                        },
                        extension_authorised_code: e.extension_authorization_code.map(|c| {
                            if c.len() != 4 {
                                return Err(tonic::Status::invalid_argument("Invalid extension authorization code".to_string()));
                            }
                            Ok(c)
                        }).transpose()?,
                    })
                }).transpose()?,
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn change_of_name(
        &self,
        request: tonic::Request<ch_ewf_grpc::change_of_name::ChangeOfName>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        if msg.proposed_name.len() < 3 || msg.proposed_name.len() > 160 {
            return Err(tonic::Status::invalid_argument("Invalid proposed name".to_string()));
        }
        if !msg.notice_given {
            return Err(tonic::Status::invalid_argument("Notice given required".to_string()));
        }

        let reply = self.form_submission(
            msg.form_submission, "ChangeOfName", "ChangeOfName",
            proto::form_submission::Form::ChangeOfName(proto::change_of_name::ChangeOfName {
                method_of_change: match ch_ewf_grpc::change_of_name::MethodOfChange::from_i32(msg.method_of_change) {
                    Some(ch_ewf_grpc::change_of_name::MethodOfChange::Resolution) => proto::change_of_name::MethodOfChange::Resolution,
                    Some(ch_ewf_grpc::change_of_name::MethodOfChange::Articles) => proto::change_of_name::MethodOfChange::Articles,
                    Some(ch_ewf_grpc::change_of_name::MethodOfChange::Llp) => proto::change_of_name::MethodOfChange::Llp,
                    None => return Err(tonic::Status::invalid_argument("Invalid method of change".to_string()))
                },
                proposed_company_name: msg.proposed_name.to_uppercase(),
                meeting_date: proto_to_chrono(msg.meeting_date).map(|d| d.date()),
                same_day: msg.same_day,
                notice_given: true,
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn psc_notification(
        &self,
        request: tonic::Request<ch_ewf_grpc::psc_notification::PscNotification>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let reply = self.form_submission(
            msg.form_submission, "PSCNotification", "PSCNotification",
            proto::form_submission::Form::PSCNotification(proto::psc_notification::PSCNotification {
                notification: match msg.notification {
                    Some(n) => n.try_into()?,
                    None => return Err(tonic::Status::invalid_argument("Notification required".to_string()))
                },
                nature_of_control: match msg.nature_of_control {
                    Some(n) => n.try_into()?,
                    None => return Err(tonic::Status::invalid_argument("Nature of control required".to_string()))
                },
                notification_date: match proto_to_chrono(msg.notification_date) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Notification date required".to_string()))
                },
                register_entry_date: match proto_to_chrono(msg.register_entry_date) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Notification date required".to_string()))
                },
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn psc_change_details(
        &self,
        request: tonic::Request<ch_ewf_grpc::psc_change_details::PscChangeDetails>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let reply = self.form_submission(
            msg.form_submission, "PSCChangeDetails", "PSCChangeDetails",
            proto::form_submission::Form::PSCChangeDetails(proto::psc_change_details::PSCChangeDetails {
                entity: match msg.entity {
                    Some(ch_ewf_grpc::psc_change_details::psc_change_details::Entity::Corporate(c)) => {
                        if c.corporate_name.is_empty() || c.corporate_name.len() > 160 {
                            return Err(tonic::Status::invalid_argument("Invalid corporate name".to_string()));
                        }

                        proto::psc_change_details::PSCEntity::Corporate(proto::psc_change_details::Corporate {
                            corporate_name: c.corporate_name,
                            change: Some(proto::psc_change_details::CorporateChange {
                                corporate_name: c.new_corporate_name.map(|n| {
                                    if n.is_empty() || n.len() > 160 {
                                        return Err(tonic::Status::invalid_argument("Invalid new corporate name".to_string()));
                                    }
                                    Ok(n)
                                }).transpose()?,
                                address: c.new_address.map(TryInto::try_into).transpose()?,
                                company_identification: c.new_corporate_identification.map(TryInto::try_into).transpose()?,
                                nature_of_controls: c.new_nature_of_controls.map(TryInto::try_into).transpose()?,
                            }),
                        })
                    }
                    Some(ch_ewf_grpc::psc_change_details::psc_change_details::Entity::LegalPerson(c)) => {
                        if c.legal_person_name.is_empty() || c.legal_person_name.len() > 160 {
                            return Err(tonic::Status::invalid_argument("Invalid legal person name".to_string()));
                        }

                        proto::psc_change_details::PSCEntity::LegalPerson(proto::psc_change_details::LegalPerson {
                            legal_person_name: c.legal_person_name,
                            change: Some(proto::psc_change_details::LegalPersonChange {
                                legal_person_name: c.new_legal_person_name.map(|n| {
                                    if n.is_empty() || n.len() > 160 {
                                        return Err(tonic::Status::invalid_argument("Invalid new legal person name".to_string()));
                                    }
                                    Ok(n)
                                }).transpose()?,
                                address: c.new_address.map(TryInto::try_into).transpose()?,
                                legal_person_identification: c.new_legal_person_identification.map(TryInto::try_into).transpose()?,
                                nature_of_controls: c.new_nature_of_controls.map(TryInto::try_into).transpose()?,
                            }),
                        })
                    }
                    Some(ch_ewf_grpc::psc_change_details::psc_change_details::Entity::Individual(c)) => {
                        proto::psc_change_details::PSCEntity::Individual(Box::new(proto::psc_change_details::Individual {
                            identification: match c.identification {
                                Some(i) => i.try_into()?,
                                None => return Err(tonic::Status::invalid_argument("Individual identification required".to_string()))
                            },
                            change: Some(proto::psc_change_details::IndividualChange {
                                name: c.new_name.map(TryInto::try_into).transpose()?,
                                service_address: c.new_service_address.map(TryInto::try_into).transpose()?,
                                residential_address: c.new_residential_address.map(TryInto::try_into).transpose()?,
                                nationality: match c.new_nationality {
                                    Some(n) => {
                                        if n.is_empty() || n.len() > 50 {
                                            return Err(tonic::Status::invalid_argument("Invalid nationality".to_string()));
                                        }
                                        Some(n)
                                    }
                                    None => None,
                                },
                                country_of_residence: match c.new_country_of_residence {
                                    Some(n) => {
                                        if n.is_empty() || n.len() > 50 {
                                            return Err(tonic::Status::invalid_argument("Invalid country of residence".to_string()));
                                        }
                                        Some(n)
                                    }
                                    None => None,
                                },
                                nature_of_controls: c.new_nature_of_controls.map(TryInto::try_into).transpose()?,
                            }),
                        }))
                    }
                    None => return Err(tonic::Status::invalid_argument("Entity changing details must be provided".to_string()))
                },
                date_of_change: match proto_to_chrono(msg.date_of_change) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Date of change required".to_string()))
                },
                register_entry_date: match proto_to_chrono(msg.register_entry_date) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Notification date required".to_string()))
                },
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn psc_cessation(
        &self,
        request: tonic::Request<ch_ewf_grpc::psc_cessation::PscCessation>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let reply = self.form_submission(
            msg.form_submission, "PSCCessation", "PSCCessation",
            proto::form_submission::Form::PSCCessation(proto::psc_cessation::PSCCessation {
                entity: match msg.entity {
                    Some(ch_ewf_grpc::psc_cessation::psc_cessation::Entity::Corporate(c)) => {
                        if c.is_empty() || c.len() > 160 {
                            return Err(tonic::Status::invalid_argument("Invalid corporate name".to_string()));
                        }

                        proto::psc_cessation::PSCEntity::Corporate(proto::psc_cessation::Corporate {
                            corporate_name: c
                        })
                    }
                    Some(ch_ewf_grpc::psc_cessation::psc_cessation::Entity::LegalPerson(c)) => {
                        if c.is_empty() || c.len() > 160 {
                            return Err(tonic::Status::invalid_argument("Invalid legal person name".to_string()));
                        }

                        proto::psc_cessation::PSCEntity::LegalPerson(proto::psc_cessation::LegalPerson {
                            legal_person_name: c
                        })
                    }
                    Some(ch_ewf_grpc::psc_cessation::psc_cessation::Entity::Individual(c)) => {
                        proto::psc_cessation::PSCEntity::Individual(c.try_into()?)
                    }
                    None => return Err(tonic::Status::invalid_argument("Entity must be provided".to_string()))
                },
                cessation_date: match proto_to_chrono(msg.cessation_date) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Cessation date required".to_string()))
                },
                register_entry_date: match proto_to_chrono(msg.register_entry_date) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Notification date required".to_string()))
                },
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn psc_statement_notification(
        &self,
        request: tonic::Request<ch_ewf_grpc::psc_statement_notification::PscStatementNotification>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let reply = self.form_submission(
            msg.form_submission, "PSCStatementNotification", "PSCStatementNotification",
            proto::form_submission::Form::PSCStatementNotification(proto::psc_statement_notification::PSCStatementNotification {
                notification: match msg.statement_notification {
                    Some(n) => n.try_into()?,
                    None => return Err(tonic::Status::invalid_argument("Statement notification required".to_string()))
                },
                register_entry_date: match proto_to_chrono(msg.register_entry_date) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Notification date required".to_string()))
                },
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn psc_statement_withdrawal(
        &self,
        request: tonic::Request<ch_ewf_grpc::psc_statement_withdrawal::PscStatementWithdrawal>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let reply = self.form_submission(
            msg.form_submission, "PSCStatementWithdrawal", "PSCStatementWithdrawal",
            proto::form_submission::Form::PSCStatementWithdrawal(proto::psc_statement_withdrawal::PSCStatementWithdrawal {
                notification: match msg.statement_notification {
                    Some(n) => n.try_into()?,
                    None => return Err(tonic::Status::invalid_argument("Statement notification required".to_string()))
                },
                withdrawal_date: match proto_to_chrono(msg.withdrawal_date) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Withdrawal date required".to_string()))
                },
                restrictions_notice_withdrawal_reason: match ch_ewf_grpc::psc_statement_withdrawal::RestrictionsNoticeWithdrawalReason::from_i32(msg.restrictions_notice_withdrawal_reason) {
                    Some(ch_ewf_grpc::psc_statement_withdrawal::RestrictionsNoticeWithdrawalReason::None) => None,
                    Some(ch_ewf_grpc::psc_statement_withdrawal::RestrictionsNoticeWithdrawalReason::WithdrawnByCompany) => Some(proto::psc_statement_withdrawal::RestrictionsNoticeWithdrawalReason::WithdrawnByCompany),
                    Some(ch_ewf_grpc::psc_statement_withdrawal::RestrictionsNoticeWithdrawalReason::WithdrawnByCourtOrder) => Some(proto::psc_statement_withdrawal::RestrictionsNoticeWithdrawalReason::WithdrawnByCourtOrder),
                    None => return Err(tonic::Status::invalid_argument("Invalid restrictions notice withdrawal reason".to_string()))
                },
                register_entry_date: match proto_to_chrono(msg.register_entry_date) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Notification date required".to_string()))
                },
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn register_elect_or_withdraw(
        &self,
        request: tonic::Request<ch_ewf_grpc::register_elect_or_withdraw::RegisterElectOrWithdraw>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let reply = self.form_submission(
            msg.form_submission, "RegisterElectOrWithdraw", "RegisterElectOrWithdraw",
            proto::form_submission::Form::RegisterElectOrWithdraw(proto::register_elect_or_withdraw::RegisterElectOrWithdraw {
                elect_or_withdraw: match ch_ewf_grpc::register_elect_or_withdraw::ElectOrWithdraw::from_i32(msg.elect_or_withdraw) {
                    Some(ch_ewf_grpc::register_elect_or_withdraw::ElectOrWithdraw::Elect) => proto::register_elect_or_withdraw::ElectOrWithdraw::Elect(true),
                    Some(ch_ewf_grpc::register_elect_or_withdraw::ElectOrWithdraw::Withdraw) => proto::register_elect_or_withdraw::ElectOrWithdraw::Withdraw(true),
                    None => return Err(tonic::Status::invalid_argument("Invalid elect or withdraw".to_string()))
                },
                register_type: match Self::map_register_type(msg.register) {
                    Some(r) => r,
                    None => return Err(tonic::Status::invalid_argument("Invalid register".to_string()))
                },
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn members_register_elect_or_withdraw(
        &self,
        request: tonic::Request<ch_ewf_grpc::members_register::MembersRegisterElectOrWithdraw>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let reply = self.form_submission(
            msg.form_submission, "MembersRegisterElectOrWithdraw", "MembersRegisterElectOrWithdraw",
            proto::form_submission::Form::MembersRegisterElectOrWithdraw(match msg.elect {
                Some(ch_ewf_grpc::members_register::members_register_elect_or_withdraw::Elect::ElectToHold(e)) => {
                    if !e.state_all_members_assented {
                        return Err(tonic::Status::invalid_argument("All members must assent".to_string()));
                    }
                    proto::members_register::MembersRegisterElectOrWithdraw::ElectToHold(proto::members_register::ElectToHold {
                        members: match e.members {
                            Some(ch_ewf_grpc::members_register::elect_to_hold::Members::MembersWithShares(m)) => {
                                if m.members.is_empty() {
                                    return Err(tonic::Status::invalid_argument("At least one member must be specified".to_string()));
                                }
                                proto::members_register::Members::MembersWithShares(m.members.into_iter().map(|m| {
                                    if m.shares_or_stock_held.is_empty() {
                                        return Err(tonic::Status::invalid_argument("At least one shares or stock held must be specified".to_string()));
                                    }
                                    if m.shares_or_stock_held.len() > 1000 {
                                        return Err(tonic::Status::invalid_argument("Invalid number of shares or stock held".to_string()));
                                    }
                                    if m.name.is_empty() {
                                        return Err(tonic::Status::invalid_argument("At least one name must be specified".to_string()));
                                    }
                                    if m.name.len() > 10 {
                                        return Err(tonic::Status::invalid_argument("Invalid number of names".to_string()));
                                    }

                                    Ok(proto::members_register::MemberWithShares {
                                        shares_or_stock_held: m.shares_or_stock_held.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
                                        name: m.name.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
                                        address: match m.address {
                                            Some(a) => a.try_into()?,
                                            None => return Err(tonic::Status::invalid_argument("Address required".to_string()))
                                        },
                                        date_registered_as_member: match proto_to_chrono(m.date_registered) {
                                            Some(d) => d.date(),
                                            None => return Err(tonic::Status::invalid_argument("Date registered as member required".to_string()))
                                        },
                                    })
                                }).collect::<Result<Vec<_>, _>>()?)
                            }
                            Some(ch_ewf_grpc::members_register::elect_to_hold::Members::MembersWithoutShares(m)) => {
                                if m.members.is_empty() {
                                    return Err(tonic::Status::invalid_argument("At least one member must be specified".to_string()));
                                }
                                proto::members_register::Members::MembersWithoutShares(m.members.into_iter().map(|m| Ok(proto::members_register::MemberWithoutShares {
                                    member: match m.member {
                                        Some(m) => m.try_into()?,
                                        None => return Err(tonic::Status::invalid_argument("Member details required".to_string()))
                                    },
                                    date_registered_as_member: match proto_to_chrono(m.date_registered) {
                                        Some(d) => d.date(),
                                        None => return Err(tonic::Status::invalid_argument("Date registered as member required".to_string()))
                                    },
                                })).collect::<Result<Vec<_>, _>>()?)
                            }
                            None => return Err(tonic::Status::invalid_argument("One of members with or without shares must be specified".to_string()))
                        },
                        state_all_members_assented: true,
                        state_overseas_registers_discontinued: e.state_overseas_registers_discontinued,
                        state_single_member_company: e.state_single_member_company,
                    })
                }
                Some(ch_ewf_grpc::members_register::members_register_elect_or_withdraw::Elect::WithdrawElectionToHold(w)) => {
                    if !w {
                        return Err(tonic::Status::invalid_argument("Withdraw election to hold must be true".to_string()));
                    }
                    proto::members_register::MembersRegisterElectOrWithdraw::WithdrawElectionToHold(true)
                }
                None => return Err(tonic::Status::invalid_argument("One of elect or withdraw must be specified".to_string()))
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn members_register_update(
        &self,
        request: tonic::Request<ch_ewf_grpc::members_register_update::MembersRegisterUpdate>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let reply = self.form_submission(
            msg.form_submission, "MembersRegisterUpdate", "MembersRegisterUpdate",
            proto::form_submission::Form::MembersRegisterUpdate(proto::members_register_update::MembersRegisterUpdate {
                members: match msg.members {
                    Some(ch_ewf_grpc::members_register_update::members_register_update::Members::MembersWithShares(s)) => {
                        if s.members.is_empty() {
                            return Err(tonic::Status::invalid_argument("Members required".to_string()));
                        }
                        proto::members_register_update::Members::MembersWithShares(s.members.into_iter().map(|m| {
                            if m.name.is_empty() {
                                return Err(tonic::Status::invalid_argument("At least one name must be specified".to_string()));
                            }
                            if m.name.len() > 10 {
                                return Err(tonic::Status::invalid_argument("Invalid number of names".to_string()));
                            }

                            Ok(proto::members_register_update::MemberWithShares {
                                name: m.name.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
                                address: match m.address {
                                    Some(a) => a.try_into()?,
                                    None => return Err(tonic::Status::invalid_argument("Address required".to_string()))
                                },
                                transfers: m.transfers.into_iter().map(|t| Ok(proto::members_register_update::Transfer {
                                    share: match t.share {
                                        Some(s) => s.try_into()?,
                                        None => return Err(tonic::Status::invalid_argument("Share required".to_string()))
                                    },
                                    transfer_date: match proto_to_chrono(t.transfer_date) {
                                        Some(s) => s.date(),
                                        None => return Err(tonic::Status::invalid_argument("Transfer date required".to_string()))
                                    },
                                })).collect::<Result<Vec<_>, _>>()?,
                                allotments: m.allotments.into_iter().map(|a| Ok(proto::members_register_update::Allotment {
                                    share: match a.share {
                                        Some(s) => s.try_into()?,
                                        None => return Err(tonic::Status::invalid_argument("Share required".to_string()))
                                    },
                                    allotment_date: match proto_to_chrono(a.allotment_date) {
                                        Some(s) => s.date(),
                                        None => return Err(tonic::Status::invalid_argument("Allotment date required".to_string()))
                                    },
                                })).collect::<Result<Vec<_>, _>>()?,
                                new_existing_or_ceased_member: match m.member_status {
                                    Some(ch_ewf_grpc::members_register_update::member_with_shares::MemberStatus::NewOrExistingMember(n)) => {
                                        if n.shares_or_stock_held.is_empty() {
                                            return Err(tonic::Status::invalid_argument("At least one shares or stock held must be specified".to_string()));
                                        }
                                        if n.shares_or_stock_held.len() > 1000 {
                                            return Err(tonic::Status::invalid_argument("Invalid number of shares or stock held".to_string()));
                                        }

                                        proto::members_register_update::NewExistingOrCeasedMemberWithShares::NewOrExistingMember(
                                            proto::members_register_update::NewOrExistingMemberWithShares {
                                                new_or_existing_member: match n.member_status {
                                                    Some(ch_ewf_grpc::members_register_update::new_or_existing_member::MemberStatus::NewMember(n)) => {
                                                        if !n {
                                                            return Err(tonic::Status::invalid_argument("New member must be true".to_string()));
                                                        }
                                                        proto::members_register_update::NewOrExistingMember::StateNewMember(true)
                                                    }
                                                    Some(ch_ewf_grpc::members_register_update::new_or_existing_member::MemberStatus::ExistingMemberDateRegistered(d)) => {
                                                        proto::members_register_update::NewOrExistingMember::DateRegisteredAsMember(proto_to_chrono(Some(d)).unwrap().date())
                                                    }
                                                    None => return Err(tonic::Status::invalid_argument("New or existing member required".to_string()))
                                                },
                                                shares_or_stock_held: n.shares_or_stock_held.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
                                            }
                                        )
                                    }
                                    Some(ch_ewf_grpc::members_register_update::member_with_shares::MemberStatus::CeasedToBeMember(c)) => {
                                        proto::members_register_update::NewExistingOrCeasedMemberWithShares::CeasedToBeMember(c.try_into()?)
                                    }
                                    None => return Err(tonic::Status::invalid_argument("Member status required".to_string()))
                                },
                            })
                        }).collect::<Result<Vec<_>, _>>()?)
                    }
                    Some(ch_ewf_grpc::members_register_update::members_register_update::Members::MembersWithoutShares(s)) => {
                        if s.members.is_empty() {
                            return Err(tonic::Status::invalid_argument("Members required".to_string()));
                        }
                        proto::members_register_update::Members::MembersWithoutShares(s.members.into_iter().map(|m| {
                            Ok(proto::members_register_update::MemberWithoutShares {
                                member: match m.member {
                                    Some(m) => m.try_into()?,
                                    None => return Err(tonic::Status::invalid_argument("Member details required".to_string()))
                                },
                                new_existing_or_ceased_member: match m.member_status {
                                    Some(ch_ewf_grpc::members_register_update::member_without_shares::MemberStatus::NewMember(n)) => {
                                        if !n {
                                            return Err(tonic::Status::invalid_argument("New member must be true".to_string()));
                                        }
                                        proto::members_register_update::NewExistingOrCeasedMemberWithoutShares::NewOrExistingMember(
                                            proto::members_register_update::NewOrExistingMember::StateNewMember(true)
                                        )
                                    }
                                    Some(ch_ewf_grpc::members_register_update::member_without_shares::MemberStatus::ExistingMemberDateRegistered(d)) => {
                                        proto::members_register_update::NewExistingOrCeasedMemberWithoutShares::NewOrExistingMember(
                                            proto::members_register_update::NewOrExistingMember::DateRegisteredAsMember(proto_to_chrono(Some(d)).unwrap().date())
                                        )
                                    }
                                    Some(ch_ewf_grpc::members_register_update::member_without_shares::MemberStatus::CeasedToBeMember(c)) => {
                                        proto::members_register_update::NewExistingOrCeasedMemberWithoutShares::CeasedToBeMember(c.try_into()?)
                                    }
                                    None => return Err(tonic::Status::invalid_argument("New, existing, or ceased required".to_string()))
                                },
                            })
                        }).collect::<Result<Vec<_>, _>>()?)
                    }
                    None => return Err(tonic::Status::invalid_argument("Members required".to_string()))
                },
                single_member: match ch_ewf_grpc::members_register_update::SingleMember::from_i32(msg.single_member) {
                    Some(ch_ewf_grpc::members_register_update::SingleMember::None) => None,
                    Some(ch_ewf_grpc::members_register_update::SingleMember::NoLongerSingleMember) =>
                        Some(proto::members_register_update::SingleMember::StateNoLongerSingleMember(true)),
                    Some(ch_ewf_grpc::members_register_update::SingleMember::BecomingSingleMember) =>
                        Some(proto::members_register_update::SingleMember::StateBecomingSingleMember(true)),
                    None => return Err(tonic::Status::invalid_argument("Single member required".to_string()))
                },
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn return_of_allotment_shares(
        &self,
        request: tonic::Request<ch_ewf_grpc::return_allotment_shares::ReturnOfAllotmentShares>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        if msg.statement_of_capital.is_empty() {
            return Err(tonic::Status::invalid_argument("Statement of capital required".to_string()));
        }

        let reply = self.form_submission(
            msg.form_submission, "ReturnOfAllotmentShares", "ReturnofAllotmentShares",
            proto::form_submission::Form::ReturnOfAllotmentShares(proto::return_allotment_shares::ReturnOfAllotmentShares {
                start_period: match proto_to_chrono(msg.start_period) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Start period required".to_string()))
                },
                end_period: proto_to_chrono(msg.end_period).map(|d| d.date()),
                statement_of_capital: proto::base_types::StatementOfCapital {
                    capital: msg.statement_of_capital.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?
                },
                allotment: msg.allotments.into_iter().map(|a| {
                    if a.share_class.is_empty() || a.share_class.len() > 50 {
                        return Err(tonic::Status::invalid_argument("Invalid share class"));
                    }
                    if a.num_shares < 0.0 || a.num_shares > 999999999999999.999999 {
                        return Err(tonic::Status::invalid_argument("Invalid number of shares"));
                    }
                    if a.amount_paid_due_per_share < 0.0 || a.amount_paid_due_per_share > 999999999999999.999999 {
                        return Err(tonic::Status::invalid_argument("Invalid amount paid due per share"));
                    }
                    if a.amount_unpaid_per_share < 0.0 || a.amount_unpaid_per_share > 999999999999999.999999 {
                        return Err(tonic::Status::invalid_argument("Invalid amount unpaid per share"));
                    }
                    if a.share_currency.len() != 3 {
                        return Err(tonic::Status::invalid_argument("Invalid share currency"));
                    }

                    Ok(proto::return_allotment_shares::Allotment {
                        allotment: proto::base_types::Allotment {
                            share_class: a.share_class,
                            num_shares: a.num_shares,
                            amount_paid_due_per_share: a.amount_paid_due_per_share,
                            amount_unpaid_per_share: a.amount_unpaid_per_share,
                            share_currency: a.share_currency,
                            share_value: a.share_value,
                            share_reference: None,
                        },
                        consideration: if a.consideration.is_empty() {
                            None
                        } else {
                            if a.consideration.len() > 2000 {
                                return Err(tonic::Status::invalid_argument("Invalid consideration"));
                            }
                            Some(a.consideration)
                        },
                    })
                }).collect::<Result<Vec<_>, _>>()?,
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn charge_registration(
        &self,
        request: tonic::Request<ch_ewf_grpc::charge_registration::ChargeRegistration>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        if msg.persons_entitled.is_empty() || msg.persons_entitled.len() > 4 {
            return Err(tonic::Status::invalid_argument("Invalid number of persons entitled".to_string()));
        }

        if msg.deed_certification_statement.is_empty() || msg.deed_certification_statement.len() > 500 {
            return Err(tonic::Status::invalid_argument("Invalid deed certification statement".to_string()));
        }
        if msg.deed_certified_by.is_empty() || msg.deed_certified_by.len() > 256 {
            return Err(tonic::Status::invalid_argument("Invalid deed certified by".to_string()));
        }

        if msg.personal_attributes.len() != 3 {
            return Err(tonic::Status::invalid_argument("Invalid number of personal attributes".to_string()));
        }

        let mut documents = vec![];

        if let Some(d) = msg.deed {
            documents.push(proto::form_submission::Document {
                data: base64::encode(d.data),
                date: proto_to_chrono(d.date).map(|d| d.date()),
                filename: if d.filename.is_empty() {
                    None
                } else {
                    if d.filename.len() > 32 {
                        return Err(tonic::Status::invalid_argument("Invalid filename".to_string()));
                    }
                    Some(d.filename)
                },
                content_type: match ch_ewf_grpc::base_types::ContentType::from_i32(d.content_type) {
                    Some(ch_ewf_grpc::base_types::ContentType::Pdf) => proto::form_submission::ContentType::Pdf,
                    Some(ch_ewf_grpc::base_types::ContentType::Pcl) => proto::form_submission::ContentType::Pcl,
                    None => return Err(tonic::Status::invalid_argument("Content type required".to_string()))
                },
                category: proto::form_submission::Category::Deed,
            });
        } else {
            return Err(tonic::Status::invalid_argument("Deed document required".to_string()));
        }

        if let Some(d) = msg.deed_supplemental {
            documents.push(proto::form_submission::Document {
                data: base64::encode(d.data),
                date: proto_to_chrono(d.date).map(|d| d.date()),
                filename: if d.filename.is_empty() {
                    None
                } else {
                    if d.filename.len() > 32 {
                        return Err(tonic::Status::invalid_argument("Invalid filename".to_string()));
                    }
                    Some(d.filename)
                },
                content_type: match ch_ewf_grpc::base_types::ContentType::from_i32(d.content_type) {
                    Some(ch_ewf_grpc::base_types::ContentType::Pdf) => proto::form_submission::ContentType::Pdf,
                    Some(ch_ewf_grpc::base_types::ContentType::Pcl) => proto::form_submission::ContentType::Pcl,
                    None => return Err(tonic::Status::invalid_argument("Content type required".to_string()))
                },
                category: proto::form_submission::Category::DeedSupplemental,
            });
        }

        let reply = self.form_submission(
            msg.form_submission, "ChargeRegistration", "ChargeRegistration",
            proto::form_submission::Form::ChargeRegistration(proto::charge_registration::ChargeRegistration {
                creation_date: match proto_to_chrono(msg.creation_date) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Creation date required".to_string()))
                },
                property_acquired_date: proto_to_chrono(msg.property_acquired_date).map(|d| d.date()),
                persons_entitled: proto::charge_registration::PersonsEntitled {
                    chargee_names: msg.persons_entitled.into_iter().map(|p| {
                        if p.len() > 256 {
                            return Err(tonic::Status::invalid_argument("Invalid person entitled".to_string()));
                        }
                        Ok(p)
                    }).collect::<Result<Vec<_>, _>>()?,
                    additional_chargees: msg.additional_persons_entitled,
                },
                charge_description: if msg.charge_description.is_empty() {
                    None
                } else {
                    if msg.charge_description.len() > 3500 {
                        return Err(tonic::Status::invalid_argument("Invalid charge description".to_string()));
                    }
                    Some(msg.charge_description)
                },
                fixed_charge: msg.fixed_charge,
                floating_charge: match ch_ewf_grpc::charge_registration::FloatingCharge::from_i32(msg.floating_charge) {
                    None => None,
                    Some(ch_ewf_grpc::charge_registration::FloatingCharge::Na) => Some(proto::charge_registration::FloatingCharge::NA),
                    Some(ch_ewf_grpc::charge_registration::FloatingCharge::CoversAll) => Some(proto::charge_registration::FloatingCharge::CoversAll),
                    Some(ch_ewf_grpc::charge_registration::FloatingCharge::DoesNotCoverAll) => Some(proto::charge_registration::FloatingCharge::DoesNotCoverAll),
                },
                negative_pledge: msg.negative_pledge,
                bare_trustee: msg.bare_trustee,
                deed_certification_statement: msg.deed_certification_statement,
                deed_certified_by: msg.deed_certified_by,
                authentication: msg.personal_attributes.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?,
            }),
            documents
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn charge_update(
        &self,
        request: tonic::Request<ch_ewf_grpc::charge_update::ChargeUpdate>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        if msg.person_delivering_name.is_empty() || msg.person_delivering_name.len() > 160 {
            return Err(tonic::Status::invalid_argument("Invalid person delivering name".to_string()));
        }
        if msg.person_interest_in_charge.is_empty() || msg.person_interest_in_charge.len() > 100 {
            return Err(tonic::Status::invalid_argument("Invalid person interest in charge".to_string()));
        }

        let reply = self.form_submission(
            msg.form_submission, "ChargeUpdate", "ChargeUpdate",
            proto::form_submission::Form::ChargeUpdate(proto::charge_update::ChargeUpdate {
                charge_id: match msg.charge_id {
                    Some(ch_ewf_grpc::charge_update::charge_update::ChargeId::ChargeCode(c)) => {
                        if c.len() < 12 || c.len() > 16 {
                            return Err(tonic::Status::invalid_argument("Invalid charge code".to_string()));
                        }
                        proto::charge_update::ChargeID::ChargeCode(c)
                    }
                    Some(ch_ewf_grpc::charge_update::charge_update::ChargeId::ExistingChargeKey(c)) => {
                        proto::charge_update::ChargeID::ExistingChargeKey(proto::charge_update::ExistingChargeKey {
                            existing_charge_key: Some(c.existing_charge_key),
                            creation_date: match proto_to_chrono(c.creation_date) {
                                Some(d) => d.date(),
                                None => return Err(tonic::Status::invalid_argument("Creation date required".to_string()))
                            },
                            instrument_description: c.instrument_description,
                            short_particulars: c.short_particulars,
                        })
                    }
                    None => return Err(tonic::Status::invalid_argument("Charge ID required".to_string()))
                },
                update: match msg.update {
                    Some(ch_ewf_grpc::charge_update::charge_update::Update::Satisfaction(s)) => {
                        match ch_ewf_grpc::charge_update::Satisfaction::from_i32(s) {
                            Some(ch_ewf_grpc::charge_update::Satisfaction::Full) =>
                                proto::charge_update::Update::Satisfaction(proto::charge_update::Satisfaction::Full),
                            Some(ch_ewf_grpc::charge_update::Satisfaction::Part) =>
                                proto::charge_update::Update::Satisfaction(proto::charge_update::Satisfaction::Part),
                            None => return Err(tonic::Status::invalid_argument("Invalid charge satisfaction".to_string()))
                        }
                    },
                    Some(ch_ewf_grpc::charge_update::charge_update::Update::FullCeaseOrRelease(s)) => {
                        match ch_ewf_grpc::charge_update::CeaseOrRelease::from_i32(s) {
                            Some(ch_ewf_grpc::charge_update::CeaseOrRelease::Cease) =>
                                proto::charge_update::Update::AllCeaseOrRelease(proto::charge_update::CeaseOrReleaseType::Cease),
                            Some(ch_ewf_grpc::charge_update::CeaseOrRelease::Release) =>
                                proto::charge_update::Update::AllCeaseOrRelease(proto::charge_update::CeaseOrReleaseType::Release),
                            Some(ch_ewf_grpc::charge_update::CeaseOrRelease::CeaseAndRelease) =>
                                proto::charge_update::Update::AllCeaseOrRelease(proto::charge_update::CeaseOrReleaseType::CeaseAndRelease),
                            None => return Err(tonic::Status::invalid_argument("Invalid charge cease or release".to_string()))
                        }
                    },
                    Some(ch_ewf_grpc::charge_update::charge_update::Update::PartCeaseOrRelease(s)) => {
                        proto::charge_update::Update::PartCeaseOrRelease(proto::charge_update::PartCeaseOrRelease {
                            cease_or_release: match ch_ewf_grpc::charge_update::CeaseOrRelease::from_i32(s.cease_or_release) {
                                Some(ch_ewf_grpc::charge_update::CeaseOrRelease::Cease) =>
                                    proto::charge_update::CeaseOrReleaseType::Cease,
                                Some(ch_ewf_grpc::charge_update::CeaseOrRelease::Release) =>
                                    proto::charge_update::CeaseOrReleaseType::Release,
                                Some(ch_ewf_grpc::charge_update::CeaseOrRelease::CeaseAndRelease) =>
                                    proto::charge_update::CeaseOrReleaseType::CeaseAndRelease,
                                None => return Err(tonic::Status::invalid_argument("Invalid charge cease or release".to_string()))
                            },
                            assets_description: s.assets_description
                        })
                    },
                    None => return Err(tonic::Status::invalid_argument("Charge update required".to_string()))
                },
                person_delivering: proto::charge_update::PersonDelivering {
                    name: msg.person_delivering_name,
                    address: match msg.person_delivering_address {
                        Some(a) => a.try_into()?,
                        None => return Err(tonic::Status::invalid_argument("Person delivering address required".to_string()))
                    },
                    interest_in_charge: msg.person_interest_in_charge
                }
            }),
            vec![]
        ).await?;

        Ok(tonic::Response::new(reply))
    }

    async fn company_incorporation(
        &self,
        request: tonic::Request<ch_ewf_grpc::company_incorporation::CompanyIncorporation>,
    ) -> Result<tonic::Response<ch_ewf_grpc::form_submission::SubmissionResponse>, tonic::Status> {
        let msg = request.into_inner();

        let conn = match self.connection.get() {
            Ok(c) => c,
            Err(err) => return Err(tonic::Status::internal(format!("Unable to access DB: {}", err)))
        };

        let submission_number = Self::gen_submission_number(&conn)?;
        let submission_id = uuid::Uuid::new_v4();

        if msg.company_name.len() < 3 || msg.company_name.len() > 160 {
            return Err(tonic::Status::invalid_argument("Invalid company name length".to_string()));
        }
        if msg.appointments.is_empty() {
            return Err(tonic::Status::invalid_argument("Appointments required".to_string()));
        }

        let mut documents = vec![];

        let has_memorandum = msg.memorandum.is_some();
        if let Some(d) = msg.memorandum {
            documents.push(proto::form_submission::Document {
                data: base64::encode(d.data),
                date: proto_to_chrono(d.date).map(|d| d.date()),
                filename: if d.filename.is_empty() {
                    None
                } else {
                    if d.filename.len() > 32 {
                        return Err(tonic::Status::invalid_argument("Invalid filename".to_string()));
                    }
                    Some(d.filename)
                },
                content_type: match ch_ewf_grpc::base_types::ContentType::from_i32(d.content_type) {
                    Some(ch_ewf_grpc::base_types::ContentType::Pdf) => proto::form_submission::ContentType::Pdf,
                    Some(ch_ewf_grpc::base_types::ContentType::Pcl) => proto::form_submission::ContentType::Pcl,
                    None => return Err(tonic::Status::invalid_argument("Content type required".to_string()))
                },
                category: proto::form_submission::Category::Memorandum,
            })
        }
        let has_same_name = msg.same_name.is_some();
        if let Some(d) = msg.same_name {
            documents.push(proto::form_submission::Document {
                data: base64::encode(d.data),
                date: proto_to_chrono(d.date).map(|d| d.date()),
                filename: if d.filename.is_empty() {
                    None
                } else {
                    if d.filename.len() > 32 {
                        return Err(tonic::Status::invalid_argument("Invalid filename".to_string()));
                    }
                    Some(d.filename)
                },
                content_type: match ch_ewf_grpc::base_types::ContentType::from_i32(d.content_type) {
                    Some(ch_ewf_grpc::base_types::ContentType::Pdf) => proto::form_submission::ContentType::Pdf,
                    Some(ch_ewf_grpc::base_types::ContentType::Pcl) => proto::form_submission::ContentType::Pcl,
                    None => return Err(tonic::Status::invalid_argument("Content type required".to_string()))
                },
                category: proto::form_submission::Category::NameExisting,
            })
        }
        let has_name_authorization = msg.name_authorization.is_some();
        if let Some(d) = msg.name_authorization {
            documents.push(proto::form_submission::Document {
                data: base64::encode(d.data),
                date: proto_to_chrono(d.date).map(|d| d.date()),
                filename: if d.filename.is_empty() {
                    None
                } else {
                    if d.filename.len() > 32 {
                        return Err(tonic::Status::invalid_argument("Invalid filename".to_string()));
                    }
                    Some(d.filename)
                },
                content_type: match ch_ewf_grpc::base_types::ContentType::from_i32(d.content_type) {
                    Some(ch_ewf_grpc::base_types::ContentType::Pdf) => proto::form_submission::ContentType::Pdf,
                    Some(ch_ewf_grpc::base_types::ContentType::Pcl) => proto::form_submission::ContentType::Pcl,
                    None => return Err(tonic::Status::invalid_argument("Content type required".to_string()))
                },
                category: proto::form_submission::Category::NameAuthentication,
            })
        }
        if let Some(d) = msg.articles_doc {
            documents.push(proto::form_submission::Document {
                data: base64::encode(d.data),
                date: proto_to_chrono(d.date).map(|d| d.date()),
                filename: if d.filename.is_empty() {
                    None
                } else {
                    if d.filename.len() > 32 {
                        return Err(tonic::Status::invalid_argument("Invalid filename".to_string()));
                    }
                    Some(d.filename)
                },
                content_type: match ch_ewf_grpc::base_types::ContentType::from_i32(d.content_type) {
                    Some(ch_ewf_grpc::base_types::ContentType::Pdf) => proto::form_submission::ContentType::Pdf,
                    Some(ch_ewf_grpc::base_types::ContentType::Pcl) => proto::form_submission::ContentType::Pcl,
                    None => return Err(tonic::Status::invalid_argument("Content type required".to_string()))
                },
                category: proto::form_submission::Category::Articles,
            })
        }
        let has_cic36 = msg.cic36.is_some();
        if let Some(d) = msg.cic36 {
            documents.push(proto::form_submission::Document {
                data: base64::encode(d.data),
                date: proto_to_chrono(d.date).map(|d| d.date()),
                filename: if d.filename.is_empty() {
                    None
                } else {
                    if d.filename.len() > 32 {
                        return Err(tonic::Status::invalid_argument("Invalid filename".to_string()));
                    }
                    Some(d.filename)
                },
                content_type: match ch_ewf_grpc::base_types::ContentType::from_i32(d.content_type) {
                    Some(ch_ewf_grpc::base_types::ContentType::Pdf) => proto::form_submission::ContentType::Pdf,
                    Some(ch_ewf_grpc::base_types::ContentType::Pcl) => proto::form_submission::ContentType::Pcl,
                    None => return Err(tonic::Status::invalid_argument("Content type required".to_string()))
                },
                category: proto::form_submission::Category::CIC36,
            })
        }

        let company_type = match ch_ewf_grpc::company_incorporation::CompanyType::from_i32(msg.company_type) {
            Some(ch_ewf_grpc::company_incorporation::CompanyType::LimitedByShares) => proto::company_incorporation::CompanyType::LimitedByShares,
            Some(ch_ewf_grpc::company_incorporation::CompanyType::LimitedByGuarantee) => proto::company_incorporation::CompanyType::LimitedByGuarantee,
            Some(ch_ewf_grpc::company_incorporation::CompanyType::LimitedByGuaranteeExempt) => proto::company_incorporation::CompanyType::LimitedByGuaranteeExempt,
            Some(ch_ewf_grpc::company_incorporation::CompanyType::Plc) => proto::company_incorporation::CompanyType::Plc,
            Some(ch_ewf_grpc::company_incorporation::CompanyType::Llp) => proto::company_incorporation::CompanyType::Llp,
            Some(ch_ewf_grpc::company_incorporation::CompanyType::LlpOnlyDesignated) => proto::company_incorporation::CompanyType::LLPOnlyDesignated,
            None => return Err(tonic::Status::invalid_argument("Company type required".to_string()))
        };

        let contact_details = msg.contact_name.is_empty() && msg.contact_number.is_empty();
        let res = match gov_talk::exec_govtalk_transaction(
            &self.sender, "CompanyIncorporation",
            proto::govtalk::GovTalkBody::FormSubmission(Box::new(proto::form_submission::FormSubmission {
                form_header: proto::form_submission::FormHeader {
                    company_number: None,
                    company_type: None,
                    company_name: msg.company_name.to_uppercase(),
                    company_authentication_code: None,
                    package_reference: self.package_reference.clone(),
                    language: match ch_ewf_grpc::form_submission::Language::from_i32(msg.language) {
                        Some(ch_ewf_grpc::form_submission::Language::English) => proto::form_submission::SubmissionLanguage::English,
                        Some(ch_ewf_grpc::form_submission::Language::Welsh) => proto::form_submission::SubmissionLanguage::Welsh,
                        None => return Err(tonic::Status::invalid_argument("Language required".to_string()))
                    },
                    form_identifier: "CompanyIncorporation".to_string(),
                    submission_number: submission_number.clone(),
                    contact_name: if contact_details {
                        None
                    } else {
                        Some(msg.contact_name)
                    },
                    contact_number: if contact_details {
                        None
                    } else {
                        Some(msg.contact_number)
                    },
                    customer_reference: msg.customer_reference.clone(),
                },
                date_signed: match proto_to_chrono(msg.date_signed) {
                    Some(d) => d.date(),
                    None => return Err(tonic::Status::invalid_argument("Date signed required".to_string()))
                },
                form: proto::form_submission::Form::CompanyIncorporation(Box::new(proto::company_incorporation::CompanyIncorporation {
                    company_type,
                    cic: has_cic36,
                    registers_held_on_public_record: if msg.registers_held_on_public_record.is_empty() {
                        None
                    } else {
                        let registers = msg.registers_held_on_public_record.into_iter().filter_map(Self::map_register_type).collect::<Vec<_>>();

                        Some(proto::company_incorporation::RegistersHeldOnPublicRecord {
                            directors: registers.contains(&proto::base_types::RegisterType::Directors),
                            directors_ura: registers.contains(&proto::base_types::RegisterType::DirectorsUsualResidentialAddress),
                            secretaries: registers.contains(&proto::base_types::RegisterType::Secretaries),
                            members: registers.contains(&proto::base_types::RegisterType::Members),
                            llp_members: registers.contains(&proto::base_types::RegisterType::LLPMembers),
                            llp_members_ura: registers.contains(&proto::base_types::RegisterType::LLPMembers),
                            psc: if registers.contains(&proto::base_types::RegisterType::PersonsOfSignificantControl) {
                                Some(proto::company_incorporation::PSCRegister {
                                    state_no_objection: true
                                })
                            } else {
                                None
                            },
                        })
                    },
                    country_of_incorporation: match ch_ewf_grpc::company_incorporation::CountryOfIncorporation::from_i32(msg.country_of_incorporation) {
                        Some(ch_ewf_grpc::company_incorporation::CountryOfIncorporation::EnglandAndWales) =>
                            proto::company_incorporation::CountryOfIncorporation::EnglandAndWales,
                        Some(ch_ewf_grpc::company_incorporation::CountryOfIncorporation::Wales) =>
                            proto::company_incorporation::CountryOfIncorporation::Wales,
                        Some(ch_ewf_grpc::company_incorporation::CountryOfIncorporation::Scotland) =>
                            proto::company_incorporation::CountryOfIncorporation::Scotland,
                        Some(ch_ewf_grpc::company_incorporation::CountryOfIncorporation::NorthernIreland) =>
                            proto::company_incorporation::CountryOfIncorporation::NorthernIreland,
                        None => return Err(tonic::Status::invalid_argument("Country of incorporation required".to_string()))
                    },
                    registered_office: match msg.registered_office {
                        Some(r) => r.try_into()?,
                        None => return Err(tonic::Status::invalid_argument("Registered office required".to_string()))
                    },
                    data_memorandum: !has_memorandum &&
                        company_type != proto::company_incorporation::CompanyType::Llp &&
                        company_type != proto::company_incorporation::CompanyType::LLPOnlyDesignated,
                    articles: match ch_ewf_grpc::company_incorporation::Articles::from_i32(msg.articles) {
                        Some(ch_ewf_grpc::company_incorporation::Articles::None) => None,
                        Some(ch_ewf_grpc::company_incorporation::Articles::ModelByShares) =>
                            Some(proto::company_incorporation::Articles::ModelByShares),
                        Some(ch_ewf_grpc::company_incorporation::Articles::ModelByGuarantee) =>
                            Some(proto::company_incorporation::Articles::ModelByGuarantee),
                        Some(ch_ewf_grpc::company_incorporation::Articles::ModelPlc) =>
                            Some(proto::company_incorporation::Articles::ModelPLC),
                        Some(ch_ewf_grpc::company_incorporation::Articles::AmendedByShares) =>
                            Some(proto::company_incorporation::Articles::AmendedByShares),
                        Some(ch_ewf_grpc::company_incorporation::Articles::AmendedByGuarantee) =>
                            Some(proto::company_incorporation::Articles::AmendedByGuarantee),
                        Some(ch_ewf_grpc::company_incorporation::Articles::AmendedPlc) =>
                            Some(proto::company_incorporation::Articles::AmendedPLC),
                        Some(ch_ewf_grpc::company_incorporation::Articles::Bespoke) =>
                            Some(proto::company_incorporation::Articles::Bespoke),
                        None => return Err(tonic::Status::invalid_argument("Articles type required".to_string()))
                    },
                    restricted_articles: msg.restricted_articles,
                    appointments: msg.appointments.into_iter().map(|a| -> Result<_, tonic::Status> {
                        Ok(proto::company_incorporation::Appointment {
                            consent_to_act: a.consent_to_act,
                            appointment: match a.appointment {
                                Some(ch_ewf_grpc::company_incorporation::appointment::Appointment::Director(d)) =>
                                    proto::company_incorporation::AppointmentType::Director(d.try_into()?),
                                Some(ch_ewf_grpc::company_incorporation::appointment::Appointment::Secretary(s)) =>
                                    proto::company_incorporation::AppointmentType::Secretary(s.try_into()?),
                                Some(ch_ewf_grpc::company_incorporation::appointment::Appointment::Member(m)) =>
                                    proto::company_incorporation::AppointmentType::Member(Box::new(m.try_into()?)),
                                None => return Err(tonic::Status::invalid_argument("Appointment type required".to_string()))
                            },
                        })
                    }).collect::<Result<Vec<_>, _>>()?,
                    pscs: match msg.psc {
                        Some(ch_ewf_grpc::company_incorporation::company_incorporation::Psc::PscStatement(s)) =>
                            proto::company_incorporation::PSCs::NoPSCStatement(match ch_ewf_grpc::company_incorporation::PscStatement::from_i32(s) {
                                Some(ch_ewf_grpc::company_incorporation::PscStatement::NoPsc) => proto::company_incorporation::NoPSCStatement::NoPSC,
                                None => return Err(tonic::Status::invalid_argument("Company level PSC statement required".to_string()))
                            }),
                        Some(ch_ewf_grpc::company_incorporation::company_incorporation::Psc::Pscs(s)) =>
                            proto::company_incorporation::PSCs::PSCs(s.pscs.into_iter().map(|p| Ok(proto::company_incorporation::Psc {
                                notification: proto::company_incorporation::PSCNotification {
                                    notification: match p.notification {
                                        Some(n) => n.try_into()?,
                                        None => return Err(tonic::Status::invalid_argument("PSC notification required".to_string()))
                                    },
                                    nature_of_control: match p.nature_of_control {
                                        Some(n) => n.try_into()?,
                                        None => return Err(tonic::Status::invalid_argument("PSC nature of controls required".to_string()))
                                    },
                                },
                            })).collect::<Result<Vec<_>, _>>()?),
                        None => return Err(tonic::Status::invalid_argument("PSCs required".to_string()))
                    },
                    statement_of_capital: if msg.statement_of_capital.is_empty() {
                        None
                    } else {
                        Some(proto::base_types::StatementOfCapital {
                            capital: msg.statement_of_capital.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?
                        })
                    },
                    subscribers: msg.subscribers.into_iter().map(|s| {
                        if s.allotments.is_empty() {
                            return Err(tonic::Status::invalid_argument("Subscriber shares required".to_string()));
                        }

                        Ok(proto::company_incorporation::Subscriber {
                            person: match s.person {
                                Some(p) => p.try_into()?,
                                None => return Err(tonic::Status::invalid_argument("Subscriber person required".to_string()))
                            },
                            shares: s.allotments.into_iter().map(|a| {
                                if a.share_class.is_empty() || a.share_class.len() > 50 {
                                    return Err(tonic::Status::invalid_argument("Invalid share class"));
                                }
                                if a.num_shares < 0.0 || a.num_shares > 999999999999999.999999 {
                                    return Err(tonic::Status::invalid_argument("Invalid number of shares"));
                                }
                                if a.amount_paid_due_per_share < 0.0 || a.amount_paid_due_per_share > 999999999999999.999999 {
                                    return Err(tonic::Status::invalid_argument("Invalid amount paid due per share"));
                                }
                                if a.amount_unpaid_per_share < 0.0 || a.amount_unpaid_per_share > 999999999999999.999999 {
                                    return Err(tonic::Status::invalid_argument("Invalid amount unpaid per share"));
                                }
                                if a.share_currency.len() != 3 {
                                    return Err(tonic::Status::invalid_argument("Invalid share currency"));
                                }

                                Ok(proto::base_types::Allotment {
                                    share_class: a.share_class,
                                    num_shares: a.num_shares,
                                    amount_paid_due_per_share: a.amount_paid_due_per_share,
                                    amount_unpaid_per_share: a.amount_unpaid_per_share,
                                    share_currency: a.share_currency,
                                    share_value: a.share_value,
                                    share_reference: if a.share_reference.is_empty() {
                                        None
                                    } else {
                                        Some(a.share_reference)
                                    },
                                })
                            }).collect::<Result<Vec<_>, _>>()?,
                            memorandum_statement: match ch_ewf_grpc::company_incorporation::MemorandumStatement::from_i32(s.memorandum_statement) {
                                Some(ch_ewf_grpc::company_incorporation::MemorandumStatement::MemberWithShares) =>
                                    Some(proto::company_incorporation::MemorandumStatement::MemberWithShares),
                                Some(ch_ewf_grpc::company_incorporation::MemorandumStatement::MemberWithoutShares) =>
                                    Some(proto::company_incorporation::MemorandumStatement::MemberWithoutShares),
                                Some(ch_ewf_grpc::company_incorporation::MemorandumStatement::NoMemorandumStatement) => None,
                                None => None
                            },
                        })
                    }).collect::<Result<Vec<_>, _>>()?,
                    guarantors: msg.guarantors.into_iter().map(|s| {
                        if s.amount_guaranteed.is_empty() || s.amount_guaranteed.len() > 100 {
                            return Err(tonic::Status::invalid_argument("Invalid amount guaranteed".to_string()));
                        }

                        Ok(proto::company_incorporation::Guarantor {
                            person: match s.person {
                                Some(p) => p.try_into()?,
                                None => return Err(tonic::Status::invalid_argument("Subscriber person required".to_string()))
                            },
                            amount_guaranteed: s.amount_guaranteed,
                            memorandum_statement: match ch_ewf_grpc::company_incorporation::MemorandumStatement::from_i32(s.memorandum_statement) {
                                Some(ch_ewf_grpc::company_incorporation::MemorandumStatement::MemberWithShares) =>
                                    Some(proto::company_incorporation::MemorandumStatement::MemberWithShares),
                                Some(ch_ewf_grpc::company_incorporation::MemorandumStatement::MemberWithoutShares) =>
                                    Some(proto::company_incorporation::MemorandumStatement::MemberWithoutShares),
                                Some(ch_ewf_grpc::company_incorporation::MemorandumStatement::NoMemorandumStatement) => None,
                                None => None
                            }
                        })
                    }).collect::<Result<Vec<_>, _>>()?,
                    authoriser: match msg.authorizer {
                        Some(ch_ewf_grpc::company_incorporation::company_incorporation::Authorizer::Agent(a)) => {
                            proto::company_incorporation::Authoriser::Agent(proto::company_incorporation::Agent {
                                authoriser: match a.authorizer {
                                    Some(a) => a.try_into()?,
                                    None => return Err(tonic::Status::invalid_argument("Agent authorization required".to_string()))
                                },
                                address: match a.address {
                                    Some(a) => a.try_into()?,
                                    None => return Err(tonic::Status::invalid_argument("Agent address required".to_string()))
                                },
                            })
                        }
                        Some(ch_ewf_grpc::company_incorporation::company_incorporation::Authorizer::Solicitor(s)) => {
                            proto::company_incorporation::Authoriser::Solicitor(s.try_into()?)
                        }
                        Some(ch_ewf_grpc::company_incorporation::company_incorporation::Authorizer::Member(m)) => {
                            proto::company_incorporation::Authoriser::Member(m.try_into()?)
                        }
                        Some(ch_ewf_grpc::company_incorporation::company_incorporation::Authorizer::AuthorizerSubscribers(s)) => {
                            proto::company_incorporation::Authoriser::Subscribers(proto::company_incorporation::AuthoriserSubscribers {
                                subscribers: s.subscribers.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?
                            })
                        }
                        None => return Err(tonic::Status::invalid_argument("Authorizer required".to_string()))
                    },
                    same_day: msg.same_day,
                    same_name: has_same_name,
                    name_authorisation: has_name_authorization,
                    reject_reference: msg.reject_reference.map(|r| {
                        if r.len() < 8 || r.len() > 9 {
                            return Err(tonic::Status::invalid_argument("Invalid reject reference".to_string()));
                        }
                        Ok(r)
                    }).transpose()?,
                    sic_codes: if msg.sic_codes.is_empty() {
                        None
                    } else {
                        Some(proto::base_types::SICCodes {
                            codes: msg.sic_codes.into_iter().map(|sic| {
                                if sic.len() > 5 || sic.len() < 4 || sic.chars().map(|c| c.is_numeric()).any(|x| !x) {
                                    Err(tonic::Status::invalid_argument("Invalid SIC code".to_string()))
                                } else {
                                    Ok(sic)
                                }
                            }).collect::<Result<Vec<_>, _>>()?
                        })
                    },
                    single_member_company: msg.single_member_company,
                })),
                additional_information: msg.corporation_tax_registration.map(|r| Ok(proto::form_submission::AdditionalInformation::CorporationTaxInformation(
                    proto::corporation_tax_information::CorporationTaxInformation {
                        abbreviated_company_name: if r.abbreviated_company_name.is_empty() {
                            None
                        } else {
                            if r.abbreviated_company_name.len() > 56 {
                                return Err(tonic::Status::invalid_argument("Invalid abbreviated company name".to_string()));
                            }
                            Some(r.abbreviated_company_name)
                        },
                        first_accounting_period_start_date: match proto_to_chrono(r.first_accounting_period_start_date) {
                            Some(d) => d.date(),
                            None => return Err(tonic::Status::invalid_argument("First accounting period start date required".to_string()))
                        },
                        accounts_made_up_date: match proto_to_chrono(r.accounts_made_up_date) {
                            Some(d) => d.date(),
                            None => return Err(tonic::Status::invalid_argument("Accounts made up date required".to_string()))
                        },
                        ct61_may_apply: r.ct61_may_apply,
                        principal_place_of_business: match r.principal_place_of_business {
                            Some(ch_ewf_grpc::company_incorporation::corporation_tax_registration::PrincipalPlaceOfBusiness::PrincipalPlaceOfBusinessSameAsRegisteredOffice(p)) => {
                                if !p {
                                    return Err(tonic::Status::invalid_argument("Same as registered office must be true".to_string()));
                                }
                                proto::corporation_tax_information::PrincipalPlaceOfBusiness::SameAsRegisteredOffice(true)
                            }
                            Some(ch_ewf_grpc::company_incorporation::corporation_tax_registration::PrincipalPlaceOfBusiness::PrincipalPlaceOfBusinessAddress(a)) => {
                                proto::corporation_tax_information::PrincipalPlaceOfBusiness::Address(a.try_into()?)
                            }
                            None => return Err(tonic::Status::invalid_argument("Principal place of business required".to_string()))
                        },
                        taken_over_business: r.taken_over_business.map(|t| {
                            if t.previous_business_name.is_empty() || t.previous_business_name.len() > 100 {
                                return Err(tonic::Status::invalid_argument("Invalid previous business name".to_string()));
                            }
                            if t.previous_owner_name.is_empty() || t.previous_owner_name.len() > 100 {
                                return Err(tonic::Status::invalid_argument("Invalid previous owner name".to_string()));
                            }

                            Ok(proto::corporation_tax_information::TakenOverBusiness {
                                previous_business: proto::corporation_tax_information::PreviousBusiness {
                                    business_name: t.previous_business_name,
                                    business_type: if t.previous_business_type.is_empty() {
                                        None
                                    } else {
                                        if t.previous_business_type.len() > 50 {
                                            return Err(tonic::Status::invalid_argument("Invalid previous business type".to_string()));
                                        }
                                        Some(t.previous_business_type)
                                    },
                                    company_registration_number: if t.previous_company_registration_number.is_empty() {
                                        None
                                    } else {
                                        if t.previous_company_registration_number.len() > 8 {
                                            return Err(tonic::Status::invalid_argument("Invalid previous company registration number".to_string()));
                                        }
                                        Some(t.previous_company_registration_number)
                                    },
                                    address: match t.previous_address {
                                        Some(a) => a.try_into()?,
                                        None => return Err(tonic::Status::invalid_argument("Previous business address required".to_string()))
                                    },
                                },
                                previous_owner: proto::corporation_tax_information::PreviousOwner {
                                    owner_name: t.previous_owner_name,
                                    address: match t.previous_owner_address {
                                        Some(a) => a.try_into()?,
                                        None => return Err(tonic::Status::invalid_argument("Previous owner address required".to_string()))
                                    },
                                },
                            })
                        }).transpose()?,
                    }
                ))).transpose()?,
                documents,
            })),
        ).await {
            Ok(r) => r,
            Err(e) => {
                return Err(tonic::Status::unknown(
                    format!("Transaction ID: {}; error description: {}", e.transaction_id, e.errors.into_iter().map(|e| e.msg).collect::<Vec<_>>().join("; "))
                ));
            }
        };

        let new_submission = models::Submission {
            id: submission_id,
            ch_submission_id: submission_number.clone(),
            company_number: None,
            received_timestamp: res.gateway_timestamp.naive_utc(),
            customer_reference: msg.customer_reference,
            status: schema::Status::Pending,
            reject_reference: None,
            examiner_telephone: None,
            examiner_comment: None,
            document_id: None,
            incorporation_date: None,
            authentication_code: None,
            charge_code: None,
        };

        if let Err(err) = diesel::insert_into(schema::submissions::table)
            .values(new_submission)
            .execute(&conn) {
            return Err(tonic::Status::internal(format!("Unable to save submission to DB: {}", err)));
        }

        Ok(tonic::Response::new(ch_ewf_grpc::form_submission::SubmissionResponse {
            transaction_id: res.transaction_id,
            submission_id: submission_id.to_string(),
            ch_submission_number: submission_number,
        }))
    }
}