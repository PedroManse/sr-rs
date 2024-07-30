-- Add up migration script here
CREATE SCHEMA "ecb";
CREATE TABLE ecb.clip (
	id SERIAL NOT NULL PRIMARY KEY,
	content TEXT NOT NULL UNIQUE
);

