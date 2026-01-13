CREATE TABLE model_bindings (
    assistant_id UUID NOT NULL REFERENCES assistants(id),
    slot_name TEXT NOT NULL,
    model_profile_id UUID NOT NULL,
    PRIMARY KEY (assistant_id, slot_name)
);

ALTER TABLE assistants DROP COLUMN model_profile_id;
