-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS bounty_chat (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL,
        bounty_id UUID NOT NULL,
        nerd_id VARCHAR(255) NOT NULL,
        chat_number VARCHAR(255) NOT NULL,
        message TEXT NOT NULL,
        file_urls TEXT[] DEFAULT ARRAY[]::TEXT[],
        is_read BOOLEAN DEFAULT false,
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now(),
        FOREIGN KEY (bounty_id) REFERENCES bounty(id) ON DELETE CASCADE
    );
