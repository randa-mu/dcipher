-- Registered events table
CREATE TABLE IF NOT EXISTS registered_events (
    id UUID PRIMARY KEY NOT NULL,
    chain_id VARCHAR(20) NOT NULL, -- can't completely store u64 in INTEGER. 20 digits for int repr.
    address BLOB NOT NULL,
    block_safety INTEGER NOT NULL,
    event_name TEXT NOT NULL,
    fields_json TEXT NOT NULL  -- json encoded fields
);

-- Event occurrences table
CREATE TABLE IF NOT EXISTS event_occurrences (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    event_id UUID NOT NULL,
    block_number VARCHAR(20) NOT NULL, -- can't completely store u64 in INTEGER. 20 digits for int repr.
    block_hash BYTES NOT NULL,
    block_timestamp DATETIME NOT NULL,
    raw_log_json TEXT NOT NULL,
    fields_json TEXT NOT NULL,  -- json encoded fields
    FOREIGN KEY (event_id) REFERENCES registered_events(id) ON DELETE CASCADE
);

