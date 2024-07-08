
-- generate like 50 clusters
INSERT INTO clusters (id, region, token) VALUES
    ('factory-a', 'us-west-1', 'token-a'),
    ('factory-b', 'us-west-2', 'token-b'),
    ('factory-c', 'us-west-3', 'token-c');
    

INSERT INTO users (username, pwd_salt, pwd_hash) VALUES
    ('user-a', '$2b$12$EgYq6Sh/8Sa59v7eHbuesO','$2b$12$EgYq6Sh/8Sa59v7eHbuesOTjtetnBR4IXR7c1cn0sg9ruF2ip9USi'),
    ('user-b', '$2b$12$duuAwANRAvOhDRB8/W8TEO','$2b$12$duuAwANRAvOhDRB8/W8TEO.0ijH9YECxCTjZjW8D3Hl4BhbbiSD9G'),
    ('user-c', '$2b$12$akuPB5eSpfQ4EywURYBbVe','$2b$12$akuPB5eSpfQ4EywURYBbVe8GuS9ONi6MoLLoeu4Fg/kAj2RKpzxoK');

INSERT INTO user_clusters (user_id, cluster_id) VALUES
    ('user-a', 'factory-a'),
    ('user-b', 'factory-b'),
    ('user-c', 'factory-c');

INSERT INTO  cluster_devices (id, cluster_id) VALUES
    ('3ebd5fd0-66f8-4aab-850d-7e9d99e97a1e', 'factory-a'),
    ('device-a1', 'factory-a'),
    ('device-a2', 'factory-a'),
    ('device-a3', 'factory-a'),
    ('device-a4', 'factory-a'),
    ('device-b1', 'factory-b'),
    ('device-b2', 'factory-b'),
    ('device-b3', 'factory-b'),
    ('device-b4', 'factory-b'),
    ('device-c1', 'factory-c'),
    ('device-c2', 'factory-c'),
    ('device-c3', 'factory-c'),
    ('device-c4', 'factory-c');

INSERT INTO device_topics (device_id, category, topic ) VALUES
    ('3ebd5fd0-66f8-4aab-850d-7e9d99e97a1e', 'status', 'led'),
    ('3ebd5fd0-66f8-4aab-850d-7e9d99e97a1e', 'status', 'temperature'),
    ('3ebd5fd0-66f8-4aab-850d-7e9d99e97a1e', 'control', 'led'),
    ('3ebd5fd0-66f8-4aab-850d-7e9d99e97a1e', 'control', 'temperature');