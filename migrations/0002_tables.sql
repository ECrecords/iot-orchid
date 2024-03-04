CREATE TABLE clusters (
    id VARCHAR(255) PRIMARY KEY,
    region VARCHAR(255)
);

CREATE TABLE users (
    username VARCHAR(255) PRIMARY KEY,
    cluster_id VARCHAR(255) REFERENCES clusters(id),
    pwd_salt VARCHAR(255) NOT NULL,
    pwd_hash VARCHAR(255) NOT NULL,
    token VARCHAR(255)
);