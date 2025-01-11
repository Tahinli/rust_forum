-- Add up migration script here
CREATE TABLE IF NOT EXISTS "login"(
    user_id BIGSERIAL NOT NULL REFERENCES "user"(id),
    token VARCHAR(1024) NOT NULL,
    PRIMARY KEY (user_id, token)
);