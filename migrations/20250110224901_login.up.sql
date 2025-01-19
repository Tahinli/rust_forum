-- Add up migration script here
CREATE TABLE IF NOT EXISTS "login" (
    user_id BIGSERIAL NOT NULL REFERENCES "user" (user_id),
    token VARCHAR(1024) NOT NULL,
    token_creation_time TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    PRIMARY KEY (user_id, token)
);
