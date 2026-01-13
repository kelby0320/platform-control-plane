ALTER TABLE assistants ADD COLUMN model_profile_id UUID NOT NULL DEFAULT gen_random_uuid();

DROP TABLE model_bindings;
