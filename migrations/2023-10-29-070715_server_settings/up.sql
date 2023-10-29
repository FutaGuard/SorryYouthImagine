-- Your SQL goes here
CREATE TABLE settings (
    id UUID PRIMARY KEY,
    guest BOOL NOT NULL,
    sitename TEXT NOT NULL
)