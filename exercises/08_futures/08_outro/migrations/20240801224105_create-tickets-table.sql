-- Add migration script here
CREATE TABLE tickets (
  id serial PRIMARY KEY,
  description TEXT NOT NULL,
  title TEXT NOT NULL,
  status TEXT NOT NULL
);