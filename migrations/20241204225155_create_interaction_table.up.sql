-- Add up migration script here
CREATE TABLE IF NOT EXISTS "interaction"(
    id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(50) UNIQUE NOT NULL
);