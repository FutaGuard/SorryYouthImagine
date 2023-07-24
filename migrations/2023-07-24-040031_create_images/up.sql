-- Your SQL goes here
CREATE TABLE images (
  id UUID PRIMARY KEY,
  date TIME DEFAULT NULL,
  upload_by UUID REFERENCES users(id),
  md5 VARCHAR DEFAULT NULL,
  origin_filename VARCHAR DEFAULT NULL
)