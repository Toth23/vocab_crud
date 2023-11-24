CREATE TABLE words
(
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    word        TEXT      NOT NULL,
    translation TEXT,
    date_added  TIMESTAMP NOT NULL,
    source      TEXT
);
