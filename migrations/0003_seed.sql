INSERT INTO clusters (id) VALUES ('factory-a'), ('factory-b'), ('factory-c');

INSERT INTO users (id, cluster_id, pwd, api_token) VALUES
    ('user-a', 'factory-a', 'password-a'),
    ('user-b', 'factory-b', 'password-b'),
    ('user-c', 'factory-c', 'password-c');