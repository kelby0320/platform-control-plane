-- Add assistant_id column to chat_sessions table
ALTER TABLE chat_sessions
ADD COLUMN assistant_id UUID NOT NULL;

-- Add foreign key constraint
ALTER TABLE chat_sessions
ADD CONSTRAINT fk_chat_sessions_assistant_id
FOREIGN KEY (assistant_id) REFERENCES assistants(id);
