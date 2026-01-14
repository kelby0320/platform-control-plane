CREATE INDEX idx_chat_messages_cursor ON chat_messages (session_id, created_at DESC, id DESC);
