-- Add up migration script here
CREATE TABLE IF NOT EXISTS "interaction"(
    interaction_id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(32) UNIQUE NOT NULL
);
