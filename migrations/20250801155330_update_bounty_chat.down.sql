-- Revert bounty_chat table structure changes

-- Add back the user_id column
ALTER TABLE bounty_chat ADD COLUMN user_id UUID;

-- Update existing data (placeholder - adjust based on your logic)
UPDATE bounty_chat SET user_id = sender_id WHERE user_id IS NULL;

-- Make user_id NOT NULL
ALTER TABLE bounty_chat ALTER COLUMN user_id SET NOT NULL;

-- Add foreign key constraint for user_id
ALTER TABLE bounty_chat ADD CONSTRAINT fk_bounty_chat_user_id FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

-- Drop the new columns
ALTER TABLE bounty_chat DROP COLUMN receiver_id;
ALTER TABLE bounty_chat DROP COLUMN sender_id; 