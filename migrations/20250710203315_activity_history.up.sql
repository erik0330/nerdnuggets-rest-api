-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS activity_history (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL,
        activity_type VARCHAR(255) NOT NULL,
        description TEXT NOT NULL,
        details TEXT,
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now(),
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
    );
