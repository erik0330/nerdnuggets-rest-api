-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS milestone (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        project_id UUID NOT NULL,
        status SMALLINT DEFAULT 0,
        number SMALLINT NOT NULL,
        title VARCHAR(255) NOT NULL,
        description TEXT NOT NULL,
        deliverables TEXT,
        challenges TEXT,
        next_steps TEXT,
        file_urls TEXT[] DEFAULT ARRAY[]::TEXT[],
        proof_status SMALLINT DEFAULT 0,
        funding_amount INT,
        days_after_start INT,
        days_of_prediction INT,
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now(),
        FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
    );
