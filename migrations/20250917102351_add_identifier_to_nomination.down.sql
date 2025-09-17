-- Remove identifier field from nomination table

DROP INDEX IF EXISTS idx_nomination_identifier;
ALTER TABLE nomination DROP COLUMN identifier;
