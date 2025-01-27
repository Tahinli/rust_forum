-- Add up migration script here
CREATE TABLE IF NOT EXISTS "comment_interaction"(
    interaction_time TIMESTAMPTZ PRIMARY KEY NOT NULL UNIQUE DEFAULT NOW(),
    comment_creation_time TIMESTAMPTZ NOT NULL REFERENCES "comment"(creation_time),
    user_id BIGSERIAL NOT NULL REFERENCES "user"(user_id),
    interaction_id BIGSERIAL NOT NULL REFERENCES "interaction"(id)
);
