-- Add up migration script here

ALTER TABLE project ADD COLUMN IF NOT EXISTS nomination_id UUID REFERENCES nomination(id) ON DELETE SET NULL;

-- Create index for better query performance
CREATE INDEX IF NOT EXISTS idx_project_nomination_id ON project(nomination_id);
