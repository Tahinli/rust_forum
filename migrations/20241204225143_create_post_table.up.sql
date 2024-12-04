-- Add up migration script here
CREATE TABLE IF NOT EXISTS "post"(
    creation_time TIMESTAMPTZ PRIMARY KEY UNIQUE NOT NULL DEFAULT NOW(),
    poster_id BIGSERIAL NOT NULL REFERENCES "user"(id),
    post VARCHAR NOT NULL UNIQUE
);