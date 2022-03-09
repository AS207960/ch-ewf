CREATE TYPE status AS ENUM ('pending', 'accepted', 'rejected', 'parked', 'internal_failure');

CREATE TABLE documents (
    id UUID PRIMARY KEY,
    company_number VARCHAR NOT NULL,
    document_date DATE NOT NULL,
    document_type VARCHAR NOT NULL,
    document_id VARCHAR NOT NULL,
    document_filename VARCHAR NOT NULL,
    storage_filename VARCHAR NOT NULL
);

CREATE TABLE submissions (
    id UUID PRIMARY KEY,
    ch_submission_id VARCHAR NOT NULL,
    company_number VARCHAR,
    received_timestamp TIMESTAMP NOT NULL,
    customer_reference VARCHAR,
    status status NOT NULL,
    reject_reference VARCHAR,
    examiner_telephone VARCHAR,
    examiner_comment VARCHAR,
    document_id UUID REFERENCES documents(id),
    incorporation_date DATE,
    authentication_code VARCHAR,
    charge_code VARCHAR
);

CREATE TABLE submission_rejections (
    id UUID PRIMARY KEY,
    submission_id UUID REFERENCES submissions(id) NOT NULL,
    code INTEGER NOT NULL,
    description VARCHAR NOT NULL,
    instance_number INT
);