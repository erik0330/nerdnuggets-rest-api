-- Add up migration script here


CREATE TABLE
    IF NOT EXISTS users (
        id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
        username VARCHAR(255),
        name VARCHAR(255),
        password VARCHAR(255),
        email VARCHAR(255) NOT NULL,
        verified_email BOOLEAN DEFAULT false,
        gmail VARCHAR(255),
        roles TEXT[] DEFAULT ARRAY[]::TEXT[],
        institution VARCHAR(255),
        avatar_url VARCHAR(255),
        bio TEXT,
        tier VARCHAR(255) NOT NULL,
        nerd_balance BIGINT DEFAULT 0,
        wallet_address VARCHAR(255),
        created_at TIMESTAMPTZ DEFAULT now(),
        updated_at TIMESTAMPTZ DEFAULT now()
    );