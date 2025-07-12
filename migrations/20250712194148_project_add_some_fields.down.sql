-- Add down migration script here

ALTER TABLE dao_vote 
    DROP COLUMN IF EXISTS amount,
    DROP COLUMN IF EXISTS count_contributors;