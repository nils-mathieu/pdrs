DROP TABLE IF EXISTS accounts;

CREATE TABLE accounts (
    id INTEGER PRIMARY KEY,
    did TEXT NOT NULL,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL
) STRICT;
