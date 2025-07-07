-- Registered events table
CREATE TABLE IF NOT EXISTS registered_events (
    id UUID PRIMARY KEY NOT NULL,
    chain_id STRING NOT NULL, -- INTEGER not large enough
    address BLOB NOT NULL,
    block_safety INTEGER NOT NULL,
    event_name TEXT NOT NULL,
    fields_json TEXT NOT NULL  -- json encoded fields
);

-- Event occurrences table
CREATE TABLE IF NOT EXISTS event_occurrences (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    event_id UUID NOT NULL,
    block_number INTEGER NOT NULL,
    block_hash BYTES NOT NULL,
    block_timestamp DATETIME NOT NULL,
    raw_log_json TEXT NOT NULL,
    fields_json TEXT NOT NULL,  -- json encoded fields
    FOREIGN KEY (event_id) REFERENCES registered_events(id) ON DELETE CASCADE
);

