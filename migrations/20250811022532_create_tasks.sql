CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE tasks (
   id TEXT PRIMARY KEY DEFAULT uuid_generate_v4(),
   title TEXT NOT NULL,
   completed BOOLEAN NOT NULL DEFAULT false
);