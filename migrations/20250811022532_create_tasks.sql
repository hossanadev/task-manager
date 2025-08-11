CREATE TABLE tasks (
   id TEXT PRIMARY KEY,
   title TEXT NOT NULL,
   completed BOOLEAN NOT NULL DEFAULT false
);