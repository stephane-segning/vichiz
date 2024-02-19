-- Your SQL goes here
CREATE TABLE "provider_records"
(
    "id" BYTEA NOT NULL PRIMARY KEY,
    "provider" BYTEA NOT NULL,
    "expires"   INT8,
    "addresses" TEXT[] NOT NULL DEFAULT '{}'::TEXT[]
);

CREATE TABLE "records"
(
    "id" BYTEA NOT NULL PRIMARY KEY,
    "value" BYTEA NOT NULL,
    "publisher" VARCHAR,
    "expires"   INT8
);

