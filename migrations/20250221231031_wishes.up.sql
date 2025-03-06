-- Add up migration script here
CREATE SCHEMA "wishes";

CREATE TABLE wishes.list (
	id SERIAL NOT NULL PRIMARY KEY,
	owner_id UUID NOT NULL REFERENCES inter.accounts(id) ON DELETE CASCADE,

	list_name TEXT NOT NULL,
	show_fullfillers BOOLEAN NOT NULL DEFAULT TRUE,
	UNIQUE(list_name, owner_id)
);

CREATE TABLE wishes.wish (
	id SERIAL NOT NULL PRIMARY KEY,
	list_id SERIAL NOT NULL REFERENCES wishes.list,

	price TEXT,
	placement INT NOT NULL,
	link TEXT NOT NULL,
	text TEXT NOT NULL,
	total_amount INT NOT NULL DEFAULT 1,
	disabled BOOLEAN NOT NULL DEFAULT false,
	CONSTRAINT unique_placement UNIQUE(placement, list_id) INITIALLY DEFERRED
);

CREATE TABLE wishes.fulfillments (
	id SERIAL NOT NULL PRIMARY KEY,
	list_id SERIAL NOT NULL REFERENCES wishes.list ON DELETE CASCADE,
	wish_id SERIAL NOT NULL REFERENCES wishes.wish ON DELETE RESTRICT,

	wish_fullfiller UUID REFERENCES inter.accounts(id) ON DELETE SET NULL,
	person TEXT
);

