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

-- Cluster_Devices Table
CREATE TABLE cluster_devices (
    id VARCHAR(255) PRIMARY KEY,
    cluster_id VARCHAR(255) REFERENCES clusters(id),
    device_type VARCHAR(100),
    firmware_version VARCHAR(50),
    serial_number VARCHAR(100),
    ip_address VARCHAR(45),
    last_seen TIMESTAMP,
    connection_status VARCHAR(50),
    battery_level INTEGER,
    temperature DECIMAL
);

-- Device_Logs Table
CREATE TABLE device_logs (
    log_id SERIAL PRIMARY KEY,
    device_id VARCHAR(255) REFERENCES cluster_devices(id),
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    log_type VARCHAR(50),
    log_description TEXT
);

-- Authentication Tokens Table
CREATE TABLE authentication_tokens (
    token_id SERIAL PRIMARY KEY,
    user_id VARCHAR(255) REFERENCES users(username),
    token TEXT NOT NULL,
    expiry_date TIMESTAMP NOT NULL
);
