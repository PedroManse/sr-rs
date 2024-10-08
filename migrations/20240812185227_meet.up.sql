-- Add up migration script here
CREATE SCHEMA "meet";

CREATE TABLE meet.groups (
	id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	owner_id UUID NOT NULL REFERENCES inter.accounts(id) ON DELETE RESTRICT,
	name TEXT NOT NULL UNIQUE,
	description TEXT
);

CREATE TABLE meet.user_calendar_entries (
	id SERIAL NOT NULL PRIMARY KEY,
	owner_id UUID NOT NULL REFERENCES inter.accounts(id) ON DELETE CASCADE,
	time DATE NOT NULL,
	title TEXT NOT NULL,
	description TEXT
);
CREATE INDEX notebook_user_calendar_entries ON meet.user_calendar_entries(owner_id);

CREATE TABLE meet.notes (
	id SERIAL NOT NULL PRIMARY KEY,
	owner_id UUID NOT NULL REFERENCES inter.accounts(id) ON DELETE CASCADE,
	content TEXT NOT NULL
);
CREATE INDEX notebook_user_notes ON meet.notes(owner_id);

CREATE TABLE meet.group_users (
	group_id UUID NOT NULL REFERENCES meet.groups(id) ON DELETE CASCADE,
	user_id UUID NOT NULL REFERENCES inter.accounts(id) ON DELETE CASCADE
);

CREATE TABLE meet.group_nvites (
	invite_id SERIAL NOT NULL PRIMARY KEY,
	group_id UUID NOT NULL REFERENCES meet.groups(id) ON DELETE CASCADE,
	user_id UUID NOT NULL REFERENCES inter.accounts(id) ON DELETE CASCADE,
	UNIQUE (group_id, user_id)
);

CREATE TABLE meet.group_calendar_entries (
	id SERIAL NOT NULL PRIMARY KEY,
	group_id UUID NOT NULL REFERENCES meet.groups(id) ON DELETE CASCADE,
	time DATE NOT NULL,
	title TEXT NOT NULL,
	description TEXT
);
CREATE INDEX notebook_group_calendar_entries ON meet.group_calendar_entries(group_id);

CREATE TABLE meet.group_notes (
	id SERIAL NOT NULL PRIMARY KEY,
	group_id UUID NOT NULL REFERENCES meet.groups(id) ON DELETE CASCADE,
	content TEXT NOT NULL
);
CREATE INDEX notebook_group_notes ON meet.group_notes(group_id);
