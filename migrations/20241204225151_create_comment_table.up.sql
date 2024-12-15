-- Add up migration script here
CREATE TABLE IF NOT EXISTS "comment"(
    post_creation_time TIMESTAMPTZ NOT NULL REFERENCES "post"(creation_time),
    creation_time TIMESTAMPTZ PRIMARY KEY NOT NULL UNIQUE DEFAULT NOW(),
    user_id BIGSERIAL NOT NULL REFERENCES "user"(id),
    comment VARCHAR NOT NULL
);