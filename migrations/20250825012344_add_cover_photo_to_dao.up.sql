-- Add up migration script here

ALTER TABLE dao ADD COLUMN IF NOT EXISTS cover_photo VARCHAR(255);
