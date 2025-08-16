CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE tasks (
       id TEXT PRIMARY KEY DEFAULT uuid_generate_v4(),
       title TEXT NOT NULL,
       status TEXT NOT NULL DEFAULT 'NotStarted' CHECK (status IN ('NotStarted', ' InProgress', 'Completed')),
       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);