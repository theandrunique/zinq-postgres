CREATE TABLE IF NOT EXISTS users (
    id BIGINT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    username_updated_at TIMESTAMPTZ NOT NULL,
    display_name TEXT NOT NULL,
    bio TEXT,
    email TEXT UNIQUE NOT NULL,
    email_updated_at TIMESTAMPTZ NOT NULL,
    email_verified BOOLEAN NOT NULL,
    is_active BOOLEAN NOT NULL,
    avatar TEXT,
    created_at TIMESTAMPTZ NOT NULL,

    -- Security
    totp_key BYTEA,
    password_hash TEXT NOT NULL,
    password_updated_at TIMESTAMPTZ NOT NULL,
    sessions_ttl SMALLINT NOT NULL
);

CREATE TABLE IF NOT EXISTS user_sessions (
    id BIGINT PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    client_name TEXT,
    device_name TEXT,
    location TEXT,
    token_id UUID UNIQUE NOT NULL,
    last_refresh_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON user_sessions(user_id, token_id);
