-- Your SQL goes here
CREATE TABLE lookup (
    title VARCHAR(255) NOT NULL PRIMARY KEY,
    byteoffset INTEGER NOT NULL,
    length INTEGER NOT NULL
);