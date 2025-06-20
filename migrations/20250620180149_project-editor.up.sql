-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS project_editor (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        project_id UUID NOT NULL,
        nerd_id VARCHAR(255) NOT NULL,
        user_id UUID NOT NULL,
        status SMALLINT DEFAULT 0,
        feedback TEXT,
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now()
    );
