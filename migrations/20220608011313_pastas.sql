-- Add migration script here
CREATE TABLE IF NOT EXISTS pastas
(
    id              INTEGER   PRIMARY KEY,
    content         TEXT      NOT NULL   ,
    file            TEXT      NOT NULL   ,
    extension       TEXT      NOT NULL   ,
    "private"       BOOLEAN   NOT NULL   ,
    editable        BOOLEAN   NOT NULL   ,
    created         INTEGER   NOT NULL   ,
    expiration      INTEGER   NOT NULL   ,
    pasta_type      TEXT      NOT NULL
);