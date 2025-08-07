-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS bid_milestone_submission (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        bid_milestone_id UUID NOT NULL,
        bid_id UUID NOT NULL,
        bounty_id UUID NOT NULL,
        nerd_id VARCHAR(255) NOT NULL,
        milestone_number SMALLINT NOT NULL,
        notes TEXT,
        attached_file_urls TEXT[] DEFAULT ARRAY[]::TEXT[],
        status SMALLINT DEFAULT 0,
        feedback TEXT,
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now(),
        reviewed_at TIMESTAMPTZ,
        approved_at TIMESTAMPTZ,
        rejected_at TIMESTAMPTZ,
        FOREIGN KEY (bid_milestone_id) REFERENCES bid_milestone(id) ON DELETE CASCADE,
        FOREIGN KEY (bid_id) REFERENCES bid(id) ON DELETE CASCADE,
        FOREIGN KEY (bounty_id) REFERENCES bounty(id) ON DELETE CASCADE
    );

-- Add indexes for better performance
CREATE INDEX IF NOT EXISTS idx_bid_milestone_submission_bid_milestone_id ON bid_milestone_submission(bid_milestone_id);
CREATE INDEX IF NOT EXISTS idx_bid_milestone_submission_bid_id ON bid_milestone_submission(bid_id);
CREATE INDEX IF NOT EXISTS idx_bid_milestone_submission_bounty_id ON bid_milestone_submission(bounty_id);

-- Add comment to explain the status values
COMMENT ON COLUMN bid_milestone_submission.status IS '0=Submitted, 1=Approved, 2=Rejected';
