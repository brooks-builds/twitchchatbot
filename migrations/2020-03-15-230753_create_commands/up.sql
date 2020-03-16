-- Your SQL goes here
CREATE TABLE commands (
  id SERIAL PRIMARY KEY,
  command VARCHAR(255) NOT NULL,
  response TEXT NOT NULL,
  used INT NOT NULL DEFAULT 0
)