-- Add up migration script here
CREATE TABLE IF NOT EXISTS "contact"(
    id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL UNIQUE
);