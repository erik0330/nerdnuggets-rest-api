-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS prediction (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        nerd_id VARCHAR(255) NOT NULL,
        contract_id BIGINT NOT NULL,
        status SMALLINT DEFAULT 0,
        title VARCHAR(255) NOT NULL,
        description TEXT,
        funding_amount INT NOT NULL,
        pool_amount INT DEFAULT 0,
        yes_pool_amount INT DEFAULT 0,
        no_pool_amount INT DEFAULT 0,
        progress SMALLINT DEFAULT 0,
        count_predictors INT DEFAULT 0,
        count_view INT DEFAULT 0,

        milestone_id UUID NOT NULL,
        project_id UUID NOT NULL,
        project_nerd_id VARCHAR(255) NOT NULL,
        project_title VARCHAR(255) NOT NULL,
        user_id UUID NOT NULL,
        cover_photo VARCHAR(255),
        category UUID[] DEFAULT ARRAY[]::UUID[],
        tags TEXT[] DEFAULT ARRAY[]::TEXT[],

        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now(),
        started_at DATE NOT NULL,
        ended_at DATE NOT NULL,
        released_at TIMESTAMPTZ,
        FOREIGN KEY (milestone_id) REFERENCES milestone(id) ON DELETE CASCADE
    );