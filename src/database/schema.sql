-- V1 (OLD) SCHEMA
-- CREATE TABLE IF NOT EXISTS notes
-- (
--     id INTEGER PRIMARY KEY AUTOINCREMENT,
--     title TEXT NOT NULL,
--     type TEXT DEFAULT 'other' CHECK (TYPE IN ('todo', 'check in', 'idea', 'other')),
--     date TEXT DEFAULT CURRENT_TIMESTAMP,
--     content TEXT
-- );

-- V2 SCHEMA
CREATE TABLE IF NOT EXISTS notes
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    type TEXT DEFAULT 'other' CHECK (TYPE IN ('todo', 'check in', 'idea', 'other')),
    date TEXT DEFAULT CURRENT_TIMESTAMP,
    file_link TEXT
);