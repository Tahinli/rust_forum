-- Add up migration script here
CREATE TABLE IF NOT EXISTS "post"(
    creation_time TIMESTAMPTZ PRIMARY KEY UNIQUE NOT NULL DEFAULT NOW(),
    user_id BIGSERIAL NOT NULL REFERENCES "user"(user_id),
    post VARCHAR(8192) NOT NULL UNIQUE
);
