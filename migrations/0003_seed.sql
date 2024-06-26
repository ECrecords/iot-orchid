
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

INSERT INTO  cluster_devices (id, cluster_id, last_seen) VALUES
    ('device-a1', 'factory-a', NOW()),
    ('device-a2', 'factory-a', NOW()),
    ('device-a3', 'factory-a', NOW()),
    ('device-a4', 'factory-a', NOW()),
    ('device-b1', 'factory-b', NOW()),
    ('device-b2', 'factory-b', NOW()),
    ('device-b3', 'factory-b', NOW()),
    ('device-b4', 'factory-b', NOW()),
    ('device-c1', 'factory-c', NOW()),
    ('device-c2', 'factory-c', NOW()),
    ('device-c3', 'factory-c', NOW()),
    ('device-c4', 'factory-c', NOW());