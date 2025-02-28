-- Your SQL goes here
CREATE TABLE todos (
   id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
   title VARCHAR NOT NULL,
   completed BOOLEAN NOT NULL DEFAULT FALSE
);