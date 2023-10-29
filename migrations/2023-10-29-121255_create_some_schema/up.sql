-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    is_admin BOOLEAN DEFAULT FALSE NOT NULL,
    active BOOLEAN DEFAULT FALSE NOT NULL,
    token VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

CREATE TABLE settings (
    id UUID PRIMARY KEY,
    guest BOOL NOT NULL,
    sitename TEXT NOT NULL
);

CREATE TABLE images (
    id UUID PRIMARY KEY,
    date TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL ,
    upload_by UUID REFERENCES users(id) NOT NULL,
    md5 VARCHAR NOT NULL,
    origin_filename VARCHAR NOT NULL
)