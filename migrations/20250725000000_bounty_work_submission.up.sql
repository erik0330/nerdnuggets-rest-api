-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS bounty_work_submission (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        bounty_id UUID NOT NULL,
        bid_id UUID NOT NULL,
        nerd_id VARCHAR(255) NOT NULL,
        user_id UUID NOT NULL,
        title TEXT NOT NULL,
        description TEXT NOT NULL,
        deliverable_files TEXT[] DEFAULT ARRAY[]::TEXT[],
        additional_notes TEXT,
        status SMALLINT DEFAULT 0,
        admin_notes TEXT,
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now(),
        submitted_at TIMESTAMPTZ,
        reviewed_at TIMESTAMPTZ,
        approved_at TIMESTAMPTZ,
        rejected_at TIMESTAMPTZ,
        FOREIGN KEY (bounty_id) REFERENCES bounty(id) ON DELETE CASCADE,
        FOREIGN KEY (bid_id) REFERENCES bid(id) ON DELETE CASCADE,
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
    );

CREATE TABLE
    IF NOT EXISTS bounty_milestone_submission (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        work_submission_id UUID NOT NULL,
        milestone_number SMALLINT NOT NULL,
        title TEXT NOT NULL,
        description TEXT NOT NULL,
        deliverable_files TEXT[] DEFAULT ARRAY[]::TEXT[],
        additional_notes TEXT,
        status SMALLINT DEFAULT 0,
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now(),
        submitted_at TIMESTAMPTZ,
        reviewed_at TIMESTAMPTZ,
        approved_at TIMESTAMPTZ,
        rejected_at TIMESTAMPTZ,
        FOREIGN KEY (work_submission_id) REFERENCES bounty_work_submission(id) ON DELETE CASCADE
    );

-- Add indexes for better performance
CREATE INDEX IF NOT EXISTS idx_bounty_work_submission_bounty_id ON bounty_work_submission(bounty_id);
CREATE INDEX IF NOT EXISTS idx_bounty_work_submission_bid_id ON bounty_work_submission(bid_id);
CREATE INDEX IF NOT EXISTS idx_bounty_work_submission_user_id ON bounty_work_submission(user_id);
CREATE INDEX IF NOT EXISTS idx_bounty_work_submission_status ON bounty_work_submission(status);
CREATE INDEX IF NOT EXISTS idx_bounty_milestone_submission_work_submission_id ON bounty_milestone_submission(work_submission_id);
CREATE INDEX IF NOT EXISTS idx_bounty_milestone_submission_status ON bounty_milestone_submission(status); 