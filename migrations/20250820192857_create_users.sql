CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users_tm (
       id TEXT PRIMARY KEY DEFAULT uuid_generate_v4(),
       email TEXT UNIQUE NOT NULL,
       username TEXT UNIQUE NOT NULL,
       password TEXT NOT NULL,
       status TEXT NOT NULL DEFAULT 'Inactive' CHECK (status IN ('Active', 'Inactive')),
       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);