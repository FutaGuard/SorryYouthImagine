-- Your SQL goes here
CREATE TABLE users (
  id UUID PRIMARY KEY,
  is_admin BOOLEAN DEFAULT FALSE,
  active BOOLEAN DEFAULT FALSE
)