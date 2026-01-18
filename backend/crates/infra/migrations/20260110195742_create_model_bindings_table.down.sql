ALTER TABLE assistants ADD COLUMN model_profile_id UUID NOT NULL;

DROP TABLE model_bindings;
