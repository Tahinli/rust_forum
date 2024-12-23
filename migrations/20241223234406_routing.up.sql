-- Add up migration script here
CREATE TABLE IF NOT EXISTS "routing"(
    id BIGSERIAL PRIMARY KEY UNIQUE NOT NULL,
    endpoint VARCHAR(255) UNIQUE NOT NULL
);