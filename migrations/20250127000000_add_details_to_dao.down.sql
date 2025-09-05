-- Add down migration script here

ALTER TABLE dao DROP COLUMN IF EXISTS details;
