-- Add up migration script here
CREATE TABLE "post"(
    creation_time TIMESTAMPTZ PRIMARY KEY UNIQUE NOT NULL,
    poster_id BIGSERIAL NOT NULL REFERENCES "user"(id),
    post VARCHAR NOT NULL UNIQUE
);