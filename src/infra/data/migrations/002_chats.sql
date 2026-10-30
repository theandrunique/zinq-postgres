CREATE TABLE IF NOT EXISTS chats (
    id BIGINT PRIMARY KEY,
    chat_type SMALLINT NOT NULL,
    name TEXT,
    owner_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    image TEXT,
    last_message_id BIGINT,
    last_message_content TEXT,
    last_message_type JSONB,
    last_message_author_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    last_message_created_at TIMESTAMPTZ,
    last_message_edited_at TIMESTAMPTZ,
    permissions BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_chats_last_message ON chats(last_message_created_at DESC);

CREATE TABLE IF NOT EXISTS private_chats (
    user_id1 BIGINT,
    user_id2 BIGINT,
    chat_id BIGINT,
    PRIMARY KEY (user_id1, user_id2)
);

CREATE TABLE IF NOT EXISTS chat_members (
    chat_id BIGINT NOT NULL REFERENCES chats(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    username TEXT NOT NULL,
    display_name TEXT NOT NULL,
    avatar TEXT,
    last_read_message_id BIGINT,
    permission_overwrites BIGINT,
    is_leave BOOLEAN NOT NULL,
    PRIMARY KEY (chat_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_chat_members_user_id_chat_id ON chat_members(user_id, chat_id DESC);

CREATE TABLE IF NOT EXISTS messages (
    id BIGINT PRIMARY KEY,
    chat_id BIGINT NOT NULL REFERENCES chats(id) ON DELETE CASCADE,
    author_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    content TEXT,
    message_type JSONB NOT NULL,
    edited_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_messages_chat_id ON messages(chat_id, id DESC);

CREATE TABLE IF NOT EXISTS attachments (
    id BIGINT PRIMARY KEY,
    message_id BIGINT NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    chat_id BIGINT NOT NULL,
    storage_key TEXT NOT NULL,
    size BIGINT NOT NULL,
    filename TEXT NOT NULL,
    content_type TEXT NOT NULL,
    duration_secs REAL,
    is_spoiler BOOLEAN NOT NULL,
    placeholder TEXT,
    waveform TEXT,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_attachments_chat_id_message_id ON attachments(chat_id, message_id DESC);
CREATE INDEX IF NOT EXISTS idx_attachments_chat_id ON attachments(chat_id, id DESC);

CREATE TABLE IF NOT EXISTS message_acks (
    chat_id BIGINT NOT NULL,
    message_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,

    PRIMARY KEY (chat_id, message_id, user_id)
);

CREATE OR REPLACE FUNCTION update_chat_last_message()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE chats
    SET
        last_message_id = NEW.id,
        last_message_created_at = NEW.created_at,
        last_message_edited_at = NEW.edited_at,
        last_message_content = LEFT(NEW.content, 50),
        last_message_type = NEW.message_type,
        last_message_author_id = NEW.author_id
    WHERE id = NEW.chat_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trg_update_chat_last_message
AFTER INSERT ON messages
FOR EACH ROW
EXECUTE FUNCTION update_chat_last_message();

CREATE OR REPLACE FUNCTION update_chat_last_message_on_edit()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE chats
    SET
        last_message_content = LEFT(NEW.content, 50),
        last_message_edited_at = NEW.edited_at
    WHERE id = NEW.chat_id
      AND last_message_id = NEW.id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trg_update_chat_last_message_on_edit
AFTER UPDATE ON messages
FOR EACH ROW
EXECUTE FUNCTION update_chat_last_message_on_edit();

CREATE OR REPLACE FUNCTION update_chat_members_info_on_user_edit()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE chat_members
    SET
        username = NEW.username,
        display_name = NEW.display_name,
        avatar = NEW.avatar
    WHERE user_id = NEW.id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trg_update_chat_members_info_on_user_edit
AFTER UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_chat_members_info_on_user_edit();

CREATE OR REPLACE TRIGGER trg_update_chat_members_info_on_user_insert
AFTER INSERT ON users
FOR EACH ROW
EXECUTE FUNCTION update_chat_members_info_on_user_edit();
