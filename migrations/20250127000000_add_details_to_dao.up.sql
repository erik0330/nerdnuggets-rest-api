-- Add up migration script here

-- Add the details column to dao table
ALTER TABLE dao ADD COLUMN IF NOT EXISTS details TEXT;

-- Copy details from project to existing daos
UPDATE dao 
SET details = p.details 
FROM project p 
WHERE dao.project_id = p.id;
