
-- Add arweave_tx_id field to bounty table
ALTER TABLE bounty ADD COLUMN IF NOT EXISTS arweave_tx_id VARCHAR(255);
ALTER TABLE project ADD COLUMN IF NOT EXISTS arweave_tx_id VARCHAR(255);
ALTER TABLE milestone ADD COLUMN IF NOT EXISTS arweave_tx_id VARCHAR(255);

