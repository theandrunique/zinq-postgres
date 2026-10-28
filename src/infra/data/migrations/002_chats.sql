CREATE TABLE IF NOT EXISTS chats (
    id bigint PRIMARY KEY,
    chat_type smallint NOT NULL,

    name text,
    owner_id bigint REFERENCES users(id) ON DELETE SET NULL,
    image text,

    last_message_id bigint,
    last_message_content text,
    last_message_type jsonb,
    last_message_author_id bigint REFERENCES users(id) ON DELETE SET NULL,
    last_message_created_at timestamptz,
    last_message_edited_at timestamptz,

    permissions bigint NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_chats_last_message ON chats(last_message_created_at DESC);

CREATE TABLE IF NOT EXISTS private_chats (
    user_id1 bigint,
    user_id2 bigint,
    chat_id bigint,
    PRIMARY KEY (user_id1, user_id2)
);

CREATE TABLE IF NOT EXISTS chat_members (
    chat_id bigint NOT NULL REFERENCES chats(id) ON DELETE CASCADE,
    user_id bigint NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    username text NOT NULL,
    display_name text NOT NULL,
    avatar text,
    last_read_message_id bigint,
    permission_overwrites bigint,
    is_leave boolean NOT NULL,
    PRIMARY KEY (chat_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_chat_members_user_id_chat_id ON chat_members(user_id, chat_id DESC);

CREATE TABLE IF NOT EXISTS messages (
    id bigint PRIMARY KEY,
    chat_id bigint NOT NULL REFERENCES chats(id) ON DELETE CASCADE,
    author_id bigint REFERENCES users(id) ON DELETE SET NULL,
    content text,
    message_type jsonb NOT NULL,
    edited_at timestamptz,
    created_at timestamptz NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_messages_chat_id ON messages(chat_id, id DESC);

CREATE TABLE IF NOT EXISTS attachments (
    id bigint PRIMARY KEY,
    message_id bigint NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    chat_id bigint NOT NULL,

    storage_key text NOT NULL,
    size bigint NOT NULL,
    filename text NOT NULL,
    content_type text NOT NULL,
    duration_secs real,
    is_spoiler boolean NOT NULL,
    placeholder text,
    waveform text,
    created_at timestamptz NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_attachments_chat_id_message_id ON attachments(chat_id, message_id DESC);
CREATE INDEX IF NOT EXISTS idx_attachments_chat_id ON attachments(chat_id, id DESC);

CREATE TABLE IF NOT EXISTS message_acks (
    chat_id bigint NOT NULL,
    message_id bigint NOT NULL,
    user_id bigint NOT NULL,
    created_at timestamptz NOT NULL,

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
        last_message_content = LEFT(NEW.content, 20),
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
        last_message_content = LEFT(NEW.content, 20),
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
