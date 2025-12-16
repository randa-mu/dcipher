ALTER TABLE event_occurrences ADD COLUMN tx_hash BLOB NOT NULL DEFAULT x'0000000000000000000000000000000000000000000000000000000000000000';

DROP VIEW IF EXISTS event_occurrences_with_context;
CREATE VIEW IF NOT EXISTS event_occurrences_with_context AS
SELECT
    occurrence.id,
    occurrence.event_id,
    occurrence.block_number,
    occurrence.block_hash,
    occurrence.block_timestamp,
    occurrence.raw_log_json,
    occurrence.fields_json,
    occurrence.tx_hash,
    event.chain_id,
    event.address,
    event.event_name
FROM event_occurrences occurrence
    INNER JOIN registered_events event ON occurrence.event_id = event.id;
