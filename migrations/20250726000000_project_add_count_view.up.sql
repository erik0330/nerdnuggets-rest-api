-- Add up migration script here

ALTER TABLE project ADD COLUMN IF NOT EXISTS count_view INT DEFAULT 0;