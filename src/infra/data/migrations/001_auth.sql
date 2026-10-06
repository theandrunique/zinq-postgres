CREATE TABLE IF NOT EXISTS users (
    id bigint PRIMARY KEY,
    username text UNIQUE NOT NULL,
    username_updated_at timestamptz NOT NULL,
    display_name text NOT NULL,
    bio text,
    email text UNIQUE NOT NULL,
    email_updated_at timestamptz NOT NULL,
    email_verified boolean NOT NULL,
    is_active boolean NOT NULL,
    avatar text,
    created_at timestamptz NOT NULL,

    -- Security
    totp_key bytea,
    password_hash text NOT NULL,
    password_updated_at timestamptz NOT NULL,
    sessions_ttl smallint NOT NULL
);

CREATE TABLE IF NOT EXISTS user_sessions (
    id bigint PRIMARY KEY,
    user_id bigint NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    client_name text,
    device_name text,
    location text,
    token_id uuid UNIQUE NOT NULL,
    last_refresh_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id, token_id);
