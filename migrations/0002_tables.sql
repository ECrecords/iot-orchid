CREATE TABLE clusters (
    id VARCHAR(255) PRIMARY KEY,
    region VARCHAR(255)
);

CREATE TABLE users (
    id VARCHAR(255) PRIMARY KEY,
    cluster_id VARCHAR(255) REFERENCES clusters(id),
    pwd VARCHAR(255),
    pwd_salt uuid NOT NULL DEFAULT gen_random_uuid (),
    api_token VARCHAR(255),
    token_salt uuid NOT NULL DEFAULT gen_random_uuid ()
);