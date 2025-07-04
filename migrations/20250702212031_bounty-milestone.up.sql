-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS bounty_milestone (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        bounty_id UUID NOT NULL,
        number SMALLINT NOT NULL,
        title VARCHAR(255) NOT NULL,
        description TEXT,
        reward_amount INT,
        timeline VARCHAR(255),
        requirements TEXT[] DEFAULT ARRAY[]::TEXT[],
        deliverables TEXT[] DEFAULT ARRAY[]::TEXT[],
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now(),
        FOREIGN KEY (bounty_id) REFERENCES bounty(id) ON DELETE CASCADE
    );