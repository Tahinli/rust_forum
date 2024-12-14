-- Add up migration script here
CREATE TABLE IF NOT EXISTS "contact"(
    user_id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE
);