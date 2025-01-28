-- Add up migration script here
CREATE TABLE IF NOT EXISTS "comment"(
    comment_id BIGSERIAL NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES "user"(user_id),
    post_id BIGINT NOT NULL REFERENCES "post"(post_id),
    creation_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    comment TEXT NOT NULL
);
