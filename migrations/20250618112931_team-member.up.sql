-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS team_member (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        name VARCHAR(255) NOT NULL,
        role VARCHAR(255) NOT NULL,
        bio TEXT,
        linkedin VARCHAR(255),
        twitter VARCHAR(255),
        github VARCHAR(255),
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now()
    );