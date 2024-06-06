CREATE TABLE clusters (
    id VARCHAR(255) PRIMARY KEY,
    region VARCHAR(255),
    -- created_at TIMESTAMP WITH TIME ZONE,
    -- last_accessed TIMESTAMP WITH TIME ZONE,
    token VARCHAR(255)

);

CREATE TABLE users (
    username VARCHAR(255) PRIMARY KEY,
    pwd_salt VARCHAR(255) NOT NULL,
    pwd_hash VARCHAR(255) NOT NULL,
    token VARCHAR(255)
);

CREATE TABLE user_clusters (
    user_id VARCHAR(255) REFERENCES users(username),
    cluster_id VARCHAR(255) REFERENCES clusters(id),
    PRIMARY KEY (user_id, cluster_id)
);

CREATE TABLE cluster_devices (
    id VARCHAR(255) PRIMARY KEY,
    cluster_id VARCHAR(255) REFERENCES clusters(id),
    last_seen TIMESTAMP
);