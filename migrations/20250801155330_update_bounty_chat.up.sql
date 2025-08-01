-- Update bounty_chat table structure
-- Add new columns
ALTER TABLE bounty_chat ADD COLUMN sender_id UUID;
ALTER TABLE bounty_chat ADD COLUMN receiver_id UUID;

-- Add foreign key constraints
ALTER TABLE bounty_chat ADD CONSTRAINT fk_bounty_chat_sender_id FOREIGN KEY (sender_id) REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE bounty_chat ADD CONSTRAINT fk_bounty_chat_receiver_id FOREIGN KEY (receiver_id) REFERENCES users(id) ON DELETE CASCADE;

-- Update existing data (this is a placeholder - you'll need to determine the actual logic)
-- For now, we'll set sender_id to user_id and receiver_id to a default value
-- You may need to adjust this based on your business logic
UPDATE bounty_chat SET sender_id = user_id, receiver_id = user_id WHERE sender_id IS NULL;

-- Make new columns NOT NULL after data migration
ALTER TABLE bounty_chat ALTER COLUMN sender_id SET NOT NULL;
ALTER TABLE bounty_chat ALTER COLUMN receiver_id SET NOT NULL;

-- Drop the old user_id column
ALTER TABLE bounty_chat DROP COLUMN user_id;
