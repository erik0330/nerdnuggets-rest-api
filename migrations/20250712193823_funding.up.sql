-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS funding (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        project_id UUID NOT NULL,
        proposal_id BIGINT NOT NULL,
        number SMALLINT NOT NULL,
        user_id UUID NOT NULL,
        amount INT NOT NULL,
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now(),
        FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
    );