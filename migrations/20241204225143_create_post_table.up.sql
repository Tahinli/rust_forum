-- Add up migration script here
CREATE TABLE IF NOT EXISTS "post"(
    user_id BIGSERIAL NOT NULL REFERENCES "user"(user_id),
    creation_time TIMESTAMPTZ UNIQUE NOT NULL DEFAULT NOW(),
    post VARCHAR(8192) NOT NULL UNIQUE,
    PRIMARY KEY(user_id, creation_time)
);
