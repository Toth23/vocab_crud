CREATE TABLE examples
(
    id      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    word_id UUID NOT NULL REFERENCES words (id),
    example TEXT NOT NULL
);
