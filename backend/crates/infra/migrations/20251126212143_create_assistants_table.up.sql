CREATE TABLE assistants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    version_major INT NOT NULL,
    version_minor INT NOT NULL,
    graph_profile_id UUID NOT NULL,
    model_profile_id UUID NOT NULL,
    system_prompt TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Seed data: Default Assistant
INSERT INTO assistants (
    id,
    name,
    description,
    version_major,
    version_minor,
    graph_profile_id,
    model_profile_id,
    system_prompt
) VALUES (
    gen_random_uuid(),
    'Default Assistant',
    'System default assistant',
    0,
    1,
    gen_random_uuid(),
    gen_random_uuid(),
    'You are a helpful assistant.'
);
