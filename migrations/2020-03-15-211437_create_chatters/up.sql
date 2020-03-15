-- Your SQL goes here
CREATE TABLE chatters (
  id SERIAL PRIMARY KEY,
  username varchar(255) NOT NULL,
  chat_date DATE NOT NULL DEFAULT NOW()
)