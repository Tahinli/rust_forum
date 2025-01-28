-- Add up migration script here
CREATE TABLE IF NOT EXISTS "comment_interaction"(
    comment_id BIGINT NOT NULL REFERENCES "comment"(comment_id),
    user_id BIGINT NOT NULL REFERENCES "user"(user_id),
    interaction_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    interaction_id BIGINT NOT NULL REFERENCES "interaction"(interaction_id),
    PRIMARY KEY(comment_id, user_id)
);
