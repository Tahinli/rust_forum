-- Add up migration script here
CREATE TABLE IF NOT EXISTS "post"(
    post_id BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
    user_id BIGINT NOT NULL REFERENCES "user"(user_id),
    creation_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    post TEXT NOT NULL UNIQUE
);
