-- Add assistant_id column to chat_sessions table
ALTER TABLE chat_sessions
ADD COLUMN assistant_id UUID;

-- Update existing rows to use the default assistant
UPDATE chat_sessions
SET assistant_id = '733750f6-66bb-4365-abcc-7ee1e989b339'
WHERE assistant_id IS NULL;

-- Make the column NOT NULL
ALTER TABLE chat_sessions
ALTER COLUMN assistant_id SET NOT NULL;

-- Add foreign key constraint
ALTER TABLE chat_sessions
ADD CONSTRAINT fk_chat_sessions_assistant_id
FOREIGN KEY (assistant_id) REFERENCES assistants(id);
