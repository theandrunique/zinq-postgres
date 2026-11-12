-- Keyset pagination of a user's chats is ordered by
-- `last_message_created_at DESC NULLS LAST, id DESC`.
--
-- The existing `idx_chats_last_message` uses the implicit `NULLS FIRST`
-- ordering for DESC, so it cannot serve this ordering. Keep it and add a
-- matching index instead.
CREATE INDEX IF NOT EXISTS idx_chats_last_message_nulls_last
    ON chats (last_message_created_at DESC NULLS LAST, id DESC);
