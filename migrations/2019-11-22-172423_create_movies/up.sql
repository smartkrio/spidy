-- Your SQL goes here

CREATE TABLE movies (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    pic VARCHAR NOT NULL,
    description TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
);
