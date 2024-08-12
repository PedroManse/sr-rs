-- Add up migration script here
CREATE TABLE ecb.private (
	owner_id UUID references inter.accounts(id),
	name TEXT NOT NULL UNIQUE,
	content BYTEA NOT NULL
);
CREATE UNIQUE INDEX ecb_private_idx ON ecb.private (name);

