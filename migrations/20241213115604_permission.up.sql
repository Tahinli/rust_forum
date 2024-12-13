-- Add up migration script here
CREATE TABLE IF NOT EXISTS "permission"(
    id BIGSERIAL PRIMARY KEY UNIQUE NOT NULL,
    name VARCHAR(50) UNIQUE NOT NULL
);