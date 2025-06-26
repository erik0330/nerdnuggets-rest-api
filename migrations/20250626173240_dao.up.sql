-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS dao (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        project_id UUID NOT NULL,
        nerd_id VARCHAR(255) NOT NULL,
        proposal_id BIGINT NOT NULL,
        user_id UUID NOT NULL,
        status SMALLINT DEFAULT 0,
        title TEXT NOT NULL,
        description TEXT NOT NULL,
        funding_goal INT NOT NULL,
        count_for INT DEFAULT 0,
        count_against INT DEFAULT 0,
        count_total INT DEFAULT 0,
        amount_for INT DEFAULT 0,
        amount_against INT DEFAULT 0,
        amount_total INT DEFAULT 0,
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now(),
        FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
    );
