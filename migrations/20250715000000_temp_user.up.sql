-- Create temp_user table for email verification
CREATE TABLE IF NOT EXISTS temp_users (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    email VARCHAR(255),
    name VARCHAR(255),
    password VARCHAR(255),
    verify_type VARCHAR(50),
    passkey VARCHAR(10),
    try_limit SMALLINT DEFAULT 5,
    iat BIGINT,
    exp BIGINT,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

-- Create index on email for faster lookups
CREATE INDEX IF NOT EXISTS idx_temp_users_email ON temp_users(email); 