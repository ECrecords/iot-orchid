INSERT INTO clusters (id) VALUES ('factory-a'), ('factory-b'), ('factory-c');

INSERT INTO users (username, cluster_id, pwd_salt, pwd_hash) VALUES
    ('user-a', 'factory-a', '$2b$12$EgYq6Sh/8Sa59v7eHbuesO','$2b$12$EgYq6Sh/8Sa59v7eHbuesOTjtetnBR4IXR7c1cn0sg9ruF2ip9USi'),
    ('user-b', 'factory-b', '$2b$12$duuAwANRAvOhDRB8/W8TEO','$2b$12$duuAwANRAvOhDRB8/W8TEO.0ijH9YECxCTjZjW8D3Hl4BhbbiSD9G'),
    ('user-c', 'factory-c', '$2b$12$akuPB5eSpfQ4EywURYBbVe','$2b$12$akuPB5eSpfQ4EywURYBbVe8GuS9ONi6MoLLoeu4Fg/kAj2RKpzxoK');