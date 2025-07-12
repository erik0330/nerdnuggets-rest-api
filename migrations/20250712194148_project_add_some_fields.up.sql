-- Add up migration script here

ALTER TABLE milestone 
    ADD COLUMN IF NOT EXISTS amount INT DEFAULT 0,
    ADD COLUMN IF NOT EXISTS count_contributors INT DEFAULT 0;