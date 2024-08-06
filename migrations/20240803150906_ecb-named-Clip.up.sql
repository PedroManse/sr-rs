-- Add up migration script here

ALTER TABLE ecb.clip RENAME TO random;
CREATE TABLE ecb.named (
	id SERIAL NOT NULL PRIMARY KEY,
	name TEXT NOT NULl UNIQUE,
	content TEXT NOT NULL
);

CREATE UNIQUE INDEX ecb_named_idx ON ecb.named (name);
