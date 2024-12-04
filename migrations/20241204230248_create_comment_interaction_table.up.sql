-- Add up migration script here
CREATE TABLE IF NOT EXISTS "comment_interaction"(
    comment_creation_time TIMESTAMPTZ NOT NULL REFERENCES "comment"(creation_time),
    interaction_id BIGSERIAL NOT NULL REFERENCES "interaction"(id),
    interactor_id BIGSERIAL NOT NULL REFERENCES "user"(id),
    interaction_time TIMESTAMPTZ PRIMARY KEY NOT NULL UNIQUE
);