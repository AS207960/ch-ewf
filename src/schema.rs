#[derive(DbEnum, Serialize, Deserialize, Clone, Debug)]
pub enum Status {
    Pending,
    Accepted,
    Rejected,
    Parked,
    InternalFailure
}

table! {
    submissions (id) {
        id -> Uuid,
        ch_submission_id -> Varchar,
        company_number -> Nullable<Varchar>,
        received_timestamp -> Timestamp,
        customer_reference -> Nullable<Varchar>,
        status -> crate::schema::StatusMapping,
        reject_reference -> Nullable<Varchar>,
        examiner_telephone -> Nullable<Varchar>,
        examiner_comment -> Nullable<Varchar>,
        document_id -> Nullable<Uuid>,
        incorporation_date -> Nullable<Date>,
        authentication_code -> Nullable<Varchar>,
        charge_code -> Nullable<Varchar>,
    }
}

table! {
    submission_rejections (id) {
        id -> Uuid,
        submission_id -> Uuid,
        code -> Integer,
        description -> Varchar,
        instance_number -> Nullable<Integer>,
    }
}

table! {
    documents (id) {
        id -> Uuid,
        company_number -> Varchar,
        document_date -> Date,
        document_type -> Varchar,
        document_id -> Varchar,
        document_filename -> Varchar,
        storage_filename -> Varchar,
    }
}

joinable!(submission_rejections -> submissions (submission_id));
joinable!(submissions -> documents (document_id));

allow_tables_to_appear_in_same_query!(
    submissions,
    submission_rejections,
    documents,
);