-- Clusters Table
CREATE TABLE clusters (
    id VARCHAR(255) PRIMARY KEY,
    region VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE,
    last_accessed TIMESTAMP WITH TIME ZONE,
    token VARCHAR(255)
);

-- Users Table
CREATE TABLE users (
    username VARCHAR(255) PRIMARY KEY,
    pwd_salt VARCHAR(255) NOT NULL,
    pwd_hash VARCHAR(255) NOT NULL,
    token VARCHAR(255)
);

-- User_Clusters Table (Association Table)
CREATE TABLE user_clusters (
    user_id VARCHAR(255) REFERENCES users(username),
    cluster_id VARCHAR(255) REFERENCES clusters(id),
    PRIMARY KEY (user_id, cluster_id)
);

-- Cluster_Devices Table,
CREATE TABLE cluster_devices (
    id VARCHAR(255) PRIMARY KEY,
    cluster_id VARCHAR(255) REFERENCES clusters(id)
);

-- Device_Topics Table
CREATE TABLE device_topics (
    device_id VARCHAR(255) REFERENCES cluster_devices(id),
    category VARCHAR(255) NOT NULL CHECK (category IN ('status', 'control', 'config')),
    topic VARCHAR(255) NOT NULL,
    PRIMARY KEY (device_id, category, topic)
);