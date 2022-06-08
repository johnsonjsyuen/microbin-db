-- Add migration script here
CREATE TABLE IF NOT EXISTS pastas
(
    id              INTEGER PRIMARY KEY ,
    pasta_id        INTEGER             ,
    content         TEXT                ,
    file            TEXT                ,
    extension       TEXT                ,
    "private"       BOOLEAN             ,
    editable        BOOLEAN             ,
    created         INTEGER             ,
    expiration      INTEGER             ,
    pasta_type      TEXT
);