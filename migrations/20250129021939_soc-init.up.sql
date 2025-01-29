-- Add up migration script here
CREATE SCHEMA "soc";

CREATE TYPE item_type AS ENUM(
	'post',
	'comment',
	'gallery',
	'article',
	'repost'
);

-- allow multiple soc accounts
CREATE TABLE soc.user (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	manager_id UUID NOT NULL REFERENCES inter.accounts(id) ON DELETE RESTRICT,
	username TEXT NOT NULL UNIQUE
);

CREATE TABLE soc.item (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	poster BIGINT NOT NULL REFERENCES soc.user(id) ON DELETE CASCADE,
	posted_time TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
	item_type item_type NOT NULL
);

CREATE TABLE soc.post (
	item_id BIGINT NOT NULL UNIQUE REFERENCES soc.item(id),
	text TEXT NOT NULL
);

CREATE TABLE soc.comment (
	item_id BIGINT NOT NULL UNIQUE REFERENCES soc.item(id),
	parent BIGINT NOT NULL REFERENCES soc.item(id),
	text TEXT NOT NULL
);

CREATE TABLE soc.article (
	item_id BIGINT NOT NULL UNIQUE REFERENCES soc.item(id),
	url TEXT NOT NULL,
	title TEXT NOT NULL
);

CREATE TABLE soc.gallery (
	item_id BIGINT NOT NULL UNIQUE REFERENCES soc.item(id),
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	title TEXT NOT NULL
);

CREATE TABLE soc.gallery_photo (
	gallery BIGINT NOT NULL UNIQUE REFERENCES soc.gallery(id),
	url TEXT NOT NULL
);

CREATE TABLE soc.repost (
	item_id BIGINT NOT NULL UNIQUE REFERENCES soc.item(id),
	original BIGINT NOT NULL REFERENCES soc.item(id),
	comment TEXT NOT NULL
);

