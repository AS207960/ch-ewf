use crate::schema::*;

#[derive(Insertable, Queryable, Identifiable, AsChangeset, Clone, Debug)]
#[table_name="submissions"]
pub struct Submission {
    pub id: uuid::Uuid,
    pub ch_submission_id: String,
    pub company_number: Option<String>,
    pub received_timestamp: chrono::NaiveDateTime,
    pub status: super::schema::Status,
    pub reject_reference: Option<String>,
    pub examiner_telephone: Option<String>,
    pub examiner_comment: Option<String>,
    pub document_id: Option<uuid::Uuid>,
    pub incorporation_date: Option<chrono::NaiveDate>,
    pub authentication_code: Option<String>,
    pub charge_code: Option<String>
}

#[derive(Insertable, Queryable, Identifiable, AsChangeset, Clone, Debug)]
#[table_name="submission_rejections"]
pub struct SubmissionRejection {
    pub id: uuid::Uuid,
    pub submission_id: uuid::Uuid,
    pub code: String,
    pub description: String,
    pub instance_number: Option<String>
}

#[derive(Insertable, Queryable, Identifiable, AsChangeset, Clone, Debug)]
#[table_name="documents"]
pub struct Documents {
    pub id: uuid::Uuid,
    pub company_number: String,
    pub document_date: chrono::NaiveDate,
    pub document_type: String,
    pub document_id: String,
    pub document_filename: String,
    pub storage_filename: String,
}