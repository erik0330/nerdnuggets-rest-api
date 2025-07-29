-- Add down migration script here

ALTER TABLE project DROP COLUMN IF EXISTS count_view;