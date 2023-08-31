-- Your SQL goes here
create table activity_times(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    start_time BIGINT NOT NULL,
    end_time BIGINT,
    activity_id INTEGER REFERENCES activities(id) NOT NULL
);