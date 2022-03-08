use chrono::prelude::*;
use crate::proto;

static GATEWAY: &str = "https://xmlgw.companieshouse.gov.uk/v1-0/xmlgw/Gateway";

#[derive(Clone)]
pub struct GovTalkSender {
    email: String,
    presenter_id: String,
    presenter_code: String,
    is_test: bool,
}

impl GovTalkSender {
    pub fn new(email: &str, presenter_id: &str, presenter_code: &str, is_test: bool) -> Self {
        Self {
            email: email.to_string(),
            presenter_id: format!("{:x}", md5::compute(presenter_id.as_bytes())),
            presenter_code: format!("{:x}", md5::compute(presenter_code.as_bytes())),
            is_test,
        }
    }
}

impl From<&GovTalkSender> for proto::govtalk::GovTalkSenderDetails {
    fn from(from: &GovTalkSender) -> Self {
        proto::govtalk::GovTalkSenderDetails {
            email_address: Some(from.email.clone()),
            id_authentication: Some(proto::govtalk::GovTalkIDAuthentication {
                sender_id: Some(from.presenter_id.clone()),
                authentication: vec![proto::govtalk::GovTalkAuthentication {
                    method: proto::govtalk::GovTalkAuthenticationMethod::Clear,
                    value: from.presenter_code.clone(),
                    role: None,
                }],
            }),
        }
    }
}

#[derive(Debug)]
pub struct GovTalkErrors {
    pub transaction_id: String,
    pub errors: Vec<GovTalkError>,
}

#[derive(Debug)]
pub struct GovTalkError {
    pub raised_by: String,
    pub code: i32,
    pub msg: String,
}

#[derive(Debug)]
pub struct GovTalkResponse {
    pub transaction_id: String,
    pub gateway_timestamp: DateTime<Utc>,
    pub warnings: Vec<GovTalkError>,
    pub body: Option<proto::govtalk::GovTalkBody>,
}

pub async fn exec_govtalk_transaction(sender: &GovTalkSender, class: &str, body: proto::govtalk::GovTalkBody) -> Result<GovTalkResponse, GovTalkErrors> {
    let trans_id = format!("{:X}", uuid::Uuid::new_v4().to_simple());
    let req_msg = proto::govtalk::GovTalkRoot {
        message: proto::govtalk::GovTalkMessage {
            envelope_version: "1.0".to_string(),
            header: proto::govtalk::GovTalkHeader {
                message_details: proto::govtalk::GovTalkMessageDetails {
                    class: class.to_string(),
                    qualifier: proto::govtalk::GovTalkQualifier::Request,
                    function: None,
                    transaction_id: Some(trans_id.clone()),
                    audit_id: None,
                    correlation_id: None,
                    response_endpoint: None,
                    transformation: None,
                    gateway_test: if sender.is_test {
                        Some(1)
                    } else {
                        None
                    },
                    gateway_timestamp: None,
                },
                sender_details: Some(sender.into()),
            },
            details: proto::govtalk::GovTalkDetails {
                keys: None,
                target_details: None,
                gateway_validation: None,
                channel_routing: vec![],
                errors: None,
            },
            body: Some(body),
        }
    };
    let req_msg_str = match xml_serde::to_string(&req_msg) {
        Ok(s) => s,
        Err(e) => return Err(GovTalkErrors {
            transaction_id: trans_id,
            errors: vec![GovTalkError {
                raised_by: "XML Encoder".to_string(),
                code: 0,
                msg: e.to_string(),
            }],
        })
    };

    println!("{}", req_msg_str);

    let http_client = reqwest::Client::new();
    let res_msg_str = match http_client.post(GATEWAY)
        .body(req_msg_str)
        .header("Content-Type", "text/xml")
        .send()
        .await {
        Ok(r) => match r.text().await {
            Ok(t) => t,
            Err(e) => return Err(GovTalkErrors {
                transaction_id: trans_id,
                errors: vec![GovTalkError {
                raised_by: "HTTP client".to_string(),
                code: 0,
                msg: e.to_string(),
            }],
            })
        },
        Err(e) => return Err(GovTalkErrors {
            transaction_id: trans_id,
            errors: vec![GovTalkError {
                raised_by: "HTTP Client".to_string(),
                code: 0,
                msg: e.to_string(),
            }],
        })
    };

    println!("{}", res_msg_str);

    let res_msg: proto::govtalk::GovTalkRoot = match xml_serde::from_str(&res_msg_str) {
        Ok(s) => s,
        Err(e) => return Err(GovTalkErrors {
            transaction_id: trans_id,
            errors: vec![GovTalkError {
                raised_by: "XML Decoder".to_string(),
                code: 0,
                msg: e.to_string(),
            }],
        })
    };
    println!("{:#?}", res_msg);

    let error_vec = res_msg.message.details.errors.map_or(vec![], |es| {
        es.errors.into_iter().map(|e| GovTalkError {
            raised_by: e.raised_by,
            code: e.number.unwrap_or(0),
            msg: e.text.join("; "),
        }).collect()
    });

    if res_msg.message.header.message_details.qualifier == proto::govtalk::GovTalkQualifier::Error {
        return Err(GovTalkErrors {
            transaction_id: trans_id,
            errors: error_vec,
        });
    }

    Ok(GovTalkResponse {
        transaction_id: trans_id,
        gateway_timestamp: res_msg.message.header.message_details.gateway_timestamp.unwrap_or_else(Utc::now),
        body: res_msg.message.body,
        warnings: error_vec,
    })
}