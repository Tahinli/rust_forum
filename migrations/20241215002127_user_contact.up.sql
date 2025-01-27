-- Add up migration script here
CREATE TABLE IF NOT EXISTS "user_contact"(
    user_id BIGSERIAL NOT NULL REFERENCES "user"(user_id),
    contact_id BIGSERIAL NOT NULL REFERENCES "contact"(id),
    contact_value VARCHAR(256) NOT NULL,
    PRIMARY KEY (user_id, contact_id),
    UNIQUE (contact_id, contact_value)
);
