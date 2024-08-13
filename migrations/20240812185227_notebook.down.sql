-- Add down migration script here
DROP INDEX notebook.notebook_user_calendar_entries;
DROP INDEX notebook.notebook_user_notes;
DROP INDEX notebook.notebook_group_calendar_entries;
DROP INDEX notebook.notebook_group_notes;

DROP TABLE notebook.group_calendar_entries;
DROP TABLE notebook.group_users;
DROP TABLE notebook.group_notes;
DROP TABLE notebook.groups;

DROP TABLE notebook.user_calendar_entries;
DROP TABLE notebook.notes;
DROP SCHEMA "notebook";
