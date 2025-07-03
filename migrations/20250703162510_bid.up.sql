-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS bid (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        bounty_id UUID NOT NULL,
        nerd_id VARCHAR(255) NOT NULL,
        user_id UUID NOT NULL,
        status SMALLINT DEFAULT 0,
        title TEXT,
        description TEXT,
        bid_amount INT NOT NULL,
        timeline VARCHAR(255) NOT NULL,
        technical_approach TEXT,
        relevant_experience TEXT,
        budget_breakdown TEXT,
        upload_files TEXT[] DEFAULT ARRAY[]::TEXT[],
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now(),
        accepted_at TIMESTAMPTZ,
        rejected_at TIMESTAMPTZ,
        canceled_at TIMESTAMPTZ,
        completed_at TIMESTAMPTZ,
        FOREIGN KEY (bounty_id) REFERENCES bounty(id) ON DELETE CASCADE
    );