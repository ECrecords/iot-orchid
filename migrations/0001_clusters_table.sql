CREATE TABLE clusters (
    id VARCHAR(255) PRIMARY KEY,
    region VARCHAR(255)
);

CREATE UNIQUE INDEX idx_clusters_id ON clusters (id);