-- Add migration script here
CREATE TABLE 'users' (
    'id' INTEGER PRIMARY KEY AUTOINCREMENT,
    'username' TEXT NOT NULL UNIQUE,
    'password' VARCHAR(100) NOT NULL,
    'group' VARCHAR(100) NOT NULL,
    'displayname' TEXT NOT NULL,
    'avatar_seed' VARCHAR(100) NOT NULL
)