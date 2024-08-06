-- Add down migration script here
DROP TABLE ecb.named;
ALTER TABLE ecb.random RENAME TO clip;
