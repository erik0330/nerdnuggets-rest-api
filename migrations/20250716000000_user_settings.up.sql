-- Add user settings columns to users table

-- Profile settings (website already exists in connect fields, adding if not exists)
ALTER TABLE users 
ADD COLUMN IF NOT EXISTS website VARCHAR(255);

-- Notification settings
ALTER TABLE users 
ADD COLUMN IF NOT EXISTS email_notifications BOOLEAN DEFAULT true,
ADD COLUMN IF NOT EXISTS push_notifications BOOLEAN DEFAULT true,
ADD COLUMN IF NOT EXISTS milestone_updates BOOLEAN DEFAULT true,
ADD COLUMN IF NOT EXISTS funding_updates BOOLEAN DEFAULT true,
ADD COLUMN IF NOT EXISTS dao_proposals BOOLEAN DEFAULT false,
ADD COLUMN IF NOT EXISTS prediction_markets BOOLEAN DEFAULT true;

-- Privacy settings
ALTER TABLE users 
ADD COLUMN IF NOT EXISTS profile_visibility BOOLEAN DEFAULT true,
ADD COLUMN IF NOT EXISTS show_funding_history BOOLEAN DEFAULT true,
ADD COLUMN IF NOT EXISTS show_prediction_history BOOLEAN DEFAULT false,
ADD COLUMN IF NOT EXISTS two_factor_enabled BOOLEAN DEFAULT false;

-- Preferences settings
ALTER TABLE users 
ADD COLUMN IF NOT EXISTS dark_mode BOOLEAN DEFAULT false,
ADD COLUMN IF NOT EXISTS language VARCHAR(50) DEFAULT 'English',
ADD COLUMN IF NOT EXISTS timezone VARCHAR(50) DEFAULT 'UTC',
ADD COLUMN IF NOT EXISTS display_currency VARCHAR(50) DEFAULT 'USD($)'; 