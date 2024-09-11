-- Add down migration script here
DROP INDEX meet.notebook_user_calendar_entries;
DROP INDEX meet.notebook_user_notes;
DROP INDEX meet.notebook_group_calendar_entries;
DROP INDEX meet.notebook_group_notes;

DROP TABLE meet.group_calendar_entries;
DROP TABLE meet.group_users;
DROP TABLE meet.group_notes;
DROP TABLE meet.groups;

DROP TABLE meet.user_calendar_entries;
DROP TABLE meet.notes;
DROP SCHEMA "meet";
