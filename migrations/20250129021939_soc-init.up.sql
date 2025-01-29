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

CREATE TABLE soc.resource (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	url TEXT NOT NULL
);

CREATE TABLE soc.item (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	poster BIGINT NOT NULL REFERENCES soc.user(id) ON DELETE CASCADE,
	posted_time TIMESTAMPTZ,
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
	resource_id BIGINT NOT NULL REFERENCES soc.resource(id),
	title TEXT NOT NULL
);

CREATE TABLE soc.gallery (
	item_id BIGINT NOT NULL UNIQUE REFERENCES soc.item(id),
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	title TEXT NOT NULL
);

CREATE TABLE soc.gallery_photo (
	gallery BIGINT NOT NULL UNIQUE REFERENCES soc.gallery(id),
	resource_id BIGINT NOT NULL REFERENCES soc.resource(id)
);

CREATE TABLE soc.repost (
	item_id BIGINT NOT NULL UNIQUE REFERENCES soc.item(id),
	original BIGINT NOT NULL REFERENCES soc.item(id),
	comment TEXT NOT NULL
);

