-- Add down migration script here

-- Drop index first
DROP INDEX IF EXISTS idx_project_nomination_id;

-- Remove the column
ALTER TABLE project DROP COLUMN IF EXISTS nomination_id;
