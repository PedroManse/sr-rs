-- Add down migration script here
DROP TABLE soc.repost;
DROP TABLE soc.gallery_photo;
DROP TABLE soc.gallery;
DROP TABLE soc.article;
DROP TABLE soc.comment;
DROP TABLE soc.post;
DROP TABLE soc.item;
DROP TABLE soc.user;
DROP TYPE item_type;
DROP SCHEMA "soc";
