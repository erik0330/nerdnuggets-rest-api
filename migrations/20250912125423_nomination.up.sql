-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS nomination (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        
        -- paper info
        doi VARCHAR(255) NOT NULL UNIQUE,
        title TEXT NOT NULL,
        abstract TEXT,
        authors TEXT[] DEFAULT ARRAY[]::TEXT[],
        publisher VARCHAR(255),
        journal VARCHAR(255),
        keywords TEXT[] DEFAULT ARRAY[]::TEXT[],
        status SMALLINT DEFAULT 0, -- 0: fetched, 1: submitted, 2: under_review, 3: approved, 4: rejected
        
        -- nominator info
        nominator_id UUID REFERENCES users(id) ON DELETE SET NULL,
        nominator_name VARCHAR(255),
        nominator_email VARCHAR(255),
        nominator_institution VARCHAR(255),
        nominator_text TEXT,
        
        -- ai info
        ai_score REAL,
        ai_analysis TEXT,
        
        -- project info
        project_id UUID REFERENCES project(id) ON DELETE SET NULL,
        
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now()
    );

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_nomination_doi ON nomination(doi);
CREATE INDEX IF NOT EXISTS idx_nomination_status ON nomination(status);
CREATE INDEX IF NOT EXISTS idx_nomination_nominator_id ON nomination(nominator_id);
CREATE INDEX IF NOT EXISTS idx_nomination_project_id ON nomination(project_id);
CREATE INDEX IF NOT EXISTS idx_nomination_created_at ON nomination(created_at);
CREATE INDEX IF NOT EXISTS idx_nomination_authors ON nomination USING GIN(authors);
CREATE INDEX IF NOT EXISTS idx_nomination_keywords ON nomination USING GIN(keywords);
