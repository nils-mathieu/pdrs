DROP TABLE IF EXISTS accounts;

CREATE TABLE accounts (
    did TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    email_verified INTEGER DEFAULT 0,
    password_hash TEXT -- may be NULL when using external auth
) STRICT;
