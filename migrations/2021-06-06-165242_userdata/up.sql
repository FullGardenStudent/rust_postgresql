-- Your SQL goes here
CREATE TABLE IF NOT EXISTS userdata(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body VARCHAR not null,
    rating integer
)