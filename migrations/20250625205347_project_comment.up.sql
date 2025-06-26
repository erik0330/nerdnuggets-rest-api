-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS project_comment (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL,
        project_id UUID NOT NULL,
        nerd_id VARCHAR(255) NOT NULL,
        comment TEXT NOT NULL,
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now(),
        FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
    );
