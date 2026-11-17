-- Users are never deleted, so every message always has an author.
-- Enforce that at the schema level:
--   1) author_id cannot be NULL;
--   2) deleting a user who has messages is forbidden (RESTRICT) instead of
--      turning message authorship into NULL.
--
-- If this migration fails on SET NOT NULL, the database contains messages with
-- author_id IS NULL. Diagnose with:
--   SELECT count(*) FROM messages WHERE author_id IS NULL;

ALTER TABLE messages ALTER COLUMN author_id SET NOT NULL;

ALTER TABLE messages DROP CONSTRAINT IF EXISTS messages_author_id_fkey;

ALTER TABLE messages
    ADD CONSTRAINT messages_author_id_fkey
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE RESTRICT;
