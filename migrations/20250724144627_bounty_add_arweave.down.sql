ALTER TABLE bounty DROP COLUMN IF EXISTS arweave_tx_id;
ALTER TABLE project DROP COLUMN IF EXISTS arweave_tx_id;
ALTER TABLE milestone DROP COLUMN IF EXISTS arweave_tx_id;