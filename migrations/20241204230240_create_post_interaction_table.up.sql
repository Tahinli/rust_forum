-- Add up migration script here
CREATE TABLE IF NOT EXISTS "post_interaction"(
    interaction_time TIMESTAMPTZ PRIMARY KEY NOT NULL UNIQUE DEFAULT NOW(),
    post_creation_time TIMESTAMPTZ NOT NULL REFERENCES "post"(creation_time),
    user_id BIGSERIAL NOT NULL REFERENCES "user"(id),
    interaction_id BIGSERIAL NOT NULL REFERENCES "interaction"(id)
);