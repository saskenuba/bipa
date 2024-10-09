CREATE TABLE IF NOT EXISTS nodes
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    public_key TEXT NOT NULL,
    alias      TEXT NOT NULL,
    capacity   INTEGER NOT NULL,
    first_seen TEXT NOT NULL
)