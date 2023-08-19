-- Your SQL goes here
CREATE TABLE activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    duration BIGINT,
    created_at BIGINT NOT NULL,
    last_modified BIGINT NOT NULL
);