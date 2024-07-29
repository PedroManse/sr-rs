-- Add up migration script here
-- inter connected tables must be in inter schema
CREATE SCHEMA inter;
CREATE TABLE inter.accounts (
	id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	name TEXT NOT NULL UNIQUE,
	password BYTEA NOT NULL
);
