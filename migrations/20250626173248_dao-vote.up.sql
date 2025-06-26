-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS dao_vote (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        dao_id UUID NOT NULL,
        project_id UUID NOT NULL,
        nerd_id VARCHAR(255) NOT NULL,
        proposal_id BIGINT NOT NULL,
        user_id UUID NOT NULL,
        status SMALLINT DEFAULT 0,
        comment TEXT,
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now()
    );
