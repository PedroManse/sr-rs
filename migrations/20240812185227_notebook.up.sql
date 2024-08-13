-- Add up migration script here
CREATE SCHEMA "notebook";

CREATE TABLE notebook.groups (
	id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	owner_id UUID NOT NULL REFERENCES inter.accounts(id),
	name TEXT NOT NULL UNIQUE,
	description TEXT
);

CREATE TABLE notebook.user_calendar_entries (
	id SERIAL NOT NULL PRIMARY KEY,
	owner_id UUID NOT NULL REFERENCES inter.accounts(id),
	time DATE NOT NULL,
	title TEXT NOT NULL,
	description TEXT
);
CREATE INDEX notebook_user_calendar_entries ON notebook.user_calendar_entries(owner_id);

CREATE TABLE notebook.notes (
	id SERIAL NOT NULL PRIMARY KEY,
	owner_id UUID NOT NULL REFERENCES inter.accounts(id),
	content TEXT NOT NULL
);
CREATE INDEX notebook_user_notes ON notebook.notes(owner_id);

CREATE TABLE notebook.group_users (
	group_id UUID NOT NULL REFERENCES notebook.groups(id),
	user_id UUID NOT NULL REFERENCES inter.accounts(id)
);

CREATE TABLE notebook.group_calendar_entries (
	id SERIAL NOT NULL PRIMARY KEY,
	group_id UUID NOT NULL REFERENCES notebook.groups(id),
	time DATE NOT NULL,
	title TEXT NOT NULL,
	description TEXT
);
CREATE INDEX notebook_group_calendar_entries ON notebook.group_calendar_entries(group_id);

CREATE TABLE notebook.group_notes (
	id SERIAL NOT NULL PRIMARY KEY,
	group_id UUID NOT NULL REFERENCES notebook.groups(id),
	content TEXT NOT NULL
);
CREATE INDEX notebook_group_notes ON notebook.group_notes(group_id);

