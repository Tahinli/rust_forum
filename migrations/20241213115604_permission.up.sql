-- Add up migration script here
CREATE TABLE IF NOT EXISTS "permission"(
    id BIGSERIAL PRIMARY KEY UNIQUE NOT NULL,
    name VARCHAR(256) UNIQUE NOT NULL
);