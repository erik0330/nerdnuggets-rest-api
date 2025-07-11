-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS values (
        key VARCHAR(255) PRIMARY KEY NOT NULL,
        value VARCHAR(255) NOT NULL
    );