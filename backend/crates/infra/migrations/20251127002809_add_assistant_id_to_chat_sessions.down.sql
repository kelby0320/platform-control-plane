-- Remove foreign key constraint
ALTER TABLE chat_sessions
DROP CONSTRAINT IF EXISTS fk_chat_sessions_assistant_id;

-- Remove assistant_id column
ALTER TABLE chat_sessions
DROP COLUMN IF EXISTS assistant_id;
