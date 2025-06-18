-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS milestone (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        project_id UUID NOT NULL,
        number SMALLINT NOT NULL,
        title VARCHAR(255) NOT NULL,
        description TEXT,
        funding_amount INT,
        days_after_start INT,
        days_of_prediction INT,
        status SMALLINT DEFAULT 0,
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now()
    );
