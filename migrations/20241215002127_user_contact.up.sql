-- Add up migration script here
CREATE TABLE IF NOT EXISTS "user_contact"(
    user_id BIGSERIAL NOT NULL REFERENCES "user"(user_id),
    contact_id BIGSERIAL NOT NULL REFERENCES "contact"(contact_id),
    contact_value VARCHAR(256) NOT NULL,
    PRIMARY KEY (user_id, contact_id),
    UNIQUE (contact_id, contact_value)
);

INSERT INTO "user_contact"(user_id, contact_id, contact_value)
VALUES (0, 0, 'builder@rust_forum.com')
ON CONFLICT(user_id, contact_id) DO UPDATE SET
"contact_value" = 'builder@rust_forum.com';
