ALTER TABLE words
    ADD COLUMN user_id TEXT NOT NULL DEFAULT 'dummy-user-id';

ALTER TABLE words
    ALTER COLUMN user_id DROP DEFAULT;
