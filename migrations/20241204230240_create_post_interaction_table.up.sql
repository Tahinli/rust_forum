-- Add up migration script here
CREATE TABLE IF NOT EXISTS "post_interaction"(
    post_id BIGINT NOT NULL REFERENCES "post"(post_id),
    user_id BIGINT NOT NULL REFERENCES "user"(user_id),
    interaction_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    interaction_id BIGINT NOT NULL REFERENCES "interaction"(interaction_id),
    PRIMARY KEY(post_id, user_id)
);
