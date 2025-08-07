-- Add status column to bid_milestone table
ALTER TABLE bid_milestone 
ADD COLUMN status SMALLINT NOT NULL DEFAULT 0;

-- Add comment to explain the status values
COMMENT ON COLUMN bid_milestone.status IS '0=Pending, 1=InProgress, 2=Submitted, 3=Completed, 4=Rejected'; 