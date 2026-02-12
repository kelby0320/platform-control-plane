-- Seed data: Default Assistant
INSERT INTO assistants (
    id,
    name,
    description,
    version_major,
    version_minor,
    graph_profile_id,
    system_prompt
) VALUES (
    '733750f6-66bb-4365-abcc-7ee1e989b339',
    'Default Assistant',
    'System default assistant',
    0,
    1,
    '572d61fc-cf7a-4c15-9534-32000b1a9572',
    'You are a helpful assistant.'
);

INSERT INTO model_bindings (
    assistant_id,
    slot_name,
    model_profile_id
) VALUES (
    '733750f6-66bb-4365-abcc-7ee1e989b339',
    'main',
    '86d4ea3b-b9de-4e32-adb2-41f2f4bff0bd'
);