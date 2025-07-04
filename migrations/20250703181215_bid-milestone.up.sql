-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS bid_milestone (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        bid_id UUID NOT NULL,
        bounty_id UUID NOT NULL,
        nerd_id VARCHAR(255) NOT NULL,
        number SMALLINT NOT NULL,
        title VARCHAR(255) NOT NULL,
        description TEXT,
        amount INT,
        timeline VARCHAR(255),
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now(),
        FOREIGN KEY (bid_id) REFERENCES bid(id) ON DELETE CASCADE
    );