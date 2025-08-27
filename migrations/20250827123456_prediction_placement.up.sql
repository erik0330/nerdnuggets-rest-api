-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS prediction_placement (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        user_address VARCHAR(42) NOT NULL,
        proposal_id BIGINT NOT NULL,
        milestone_index BIGINT NOT NULL,
        predicts_success BOOLEAN NOT NULL,
        nerd_amount BIGINT NOT NULL,
        block_number BIGINT NOT NULL,
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now()
    );

-- Add indexes for better performance
CREATE INDEX IF NOT EXISTS idx_prediction_placement_user_address ON prediction_placement(user_address);
CREATE INDEX IF NOT EXISTS idx_prediction_placement_proposal_id ON prediction_placement(proposal_id);
