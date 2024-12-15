-- Add up migration script here
CREATE TABLE IF NOT EXISTS "comment_interaction"(
    comment_creation_time TIMESTAMPTZ NOT NULL REFERENCES "comment"(creation_time),
    user_id BIGSERIAL NOT NULL REFERENCES "user"(id),
    interaction_id BIGSERIAL NOT NULL REFERENCES "interaction"(id),
    interaction_time TIMESTAMPTZ PRIMARY KEY NOT NULL UNIQUE DEFAULT NOW()
);