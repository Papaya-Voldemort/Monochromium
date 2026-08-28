CREATE TABLE IF NOT EXISTS notes
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    type TEXT DEFAULT 'other' CHECK (TYPE IN ('todo', 'check in', 'idea')),
    date TEXT DEFAULT CURRENT_TIMESTAMP,
    content TEXT
);