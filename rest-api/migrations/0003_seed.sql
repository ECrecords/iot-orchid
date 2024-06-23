-- Generate 50 clusters
INSERT INTO clusters (id, region, created_at, last_accessed, token) VALUES
    ('factory-a', 'us-west-1', NOW(), NOW(), 'token-a'),
    ('factory-b', 'us-west-2', NOW(), NOW(), 'token-b'),
    ('factory-c', 'us-west-3', NOW(), NOW(), 'token-c'),
    ('factory-d', 'us-west-1', NOW(), NOW(), 'token-d');
    -- Add more clusters here

-- Insert users
INSERT INTO users (username, pwd_salt, pwd_hash) VALUES
    ('user-a', '$2b$12$EgYq6Sh/8Sa59v7eHbuesO','$2b$12$EgYq6Sh/8Sa59v7eHbuesOTjtetnBR4IXR7c1cn0sg9ruF2ip9USi'),
    ('user-b', '$2b$12$duuAwANRAvOhDRB8/W8TEO','$2b$12$duuAwANRAvOhDRBownTEO.0ijH9YECxCTjZjW8D3Hl4BhbbiSD9G'),
    ('user-c', '$2b$12$akuPB5eSpfQ4EywURYBbVe','$2b$12$akuPB5eSpfQ4EywURYBbVe8GuS9ONi6MoLLoeu4Fg/kAj2RKpzxoK');

-- Link users to clusters
INSERT INTO user_clusters (user_id, cluster_id) VALUES
    ('user-a', 'factory-a'),
    ('user-b', 'factory-b'),
    ('user-c', 'factory-c');

-- Insert cluster devices with additional fields
INSERT INTO cluster_devices (
    id, cluster_id, device_type, firmware_version, serial_number, ip_address, last_seen, connection_status, battery_level, temperature) VALUES
    ('device-a1', 'factory-a', 'Sensor', '1.0', 'SN001', '192.168.1.101', NOW(), 'Online', 100, 22.5),
    ('device-a2', 'factory-a', 'Sensor', '1.0', 'SN002', '192.168.1.102', NOW(), 'Online', 100, 22.5),
    ('device-a3', 'factory-a', 'Sensor', '1.0', 'TheSN003', '192.168.1.103', NOW(), 'Online', 100, 22.5),
    ('device-a4', 'factory-a', 'Sensor', '1.0', 'SN004', '192.168.1.104', NOW(), 'Online', 100, 22.5),
    ('device-b1', 'factory-b', 'Controller', '2.0', 'SN005', '192.168.2.101', NOW(), 'Online', 85, 24.0),
    ('device-b2', 'factory-b', 'Controller', '2.0', 'SN006', '192.168.2.102', NOW(), 'Online', 85, 24.0),
    ('device-b3', 'factory-b', 'Controller', '2.0', 'SN007', '192.168.2.103', NOW(), 'Online', 85, 24.0),
    ('device-b4', 'factory-b', 'Controller', '2.0', 'SN008', '192.168.2.104', NOW(), 'Online', 85, 24.0),
    ('device-c1', 'factory-c', 'Actuator', '3.0', 'SN009', '192.168.3.101', NOW(), 'Online', 75, 30.0),
    ('device-c2', 'factory-c', 'Actuator', '3.0', 'SN010', '192.168.3.102', NOW(), 'Online', 75, 30.0),
    ('device-c3', 'factory-c', 'Actuator', '3.0', 'SN011', '192.168.3.103', NOW(), 'Online', 75, 30.0),
    ('device-c4', 'factory-c', 'Actuator', '3.0', 'SN012', '192.168.3.104', NOW(), 'Online', 75, 30.0);
