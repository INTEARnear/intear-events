CREATE TABLE log_nep297_events_new (
    id BIGSERIAL,
    block_height BIGINT NOT NULL,
    block_timestamp TIMESTAMPTZ NOT NULL,
    transaction_id TEXT NOT NULL,
    receipt_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    predecessor_id TEXT NOT NULL,
    event_standard TEXT NOT NULL,
    event_version TEXT NOT NULL,
    event_event TEXT NOT NULL,
    event_data JSONB
);

SELECT create_hypertable('log_nep297_events_new', 'id',
    chunk_time_interval => 100000000);

CREATE INDEX idx_log_nep297_events_new_standard_event
    ON log_nep297_events_new (event_standard, event_event, id DESC);
CREATE INDEX idx_log_nep297_events_new_standard_event_account
    ON log_nep297_events_new (event_standard, event_event, account_id, id DESC);
CREATE INDEX idx_log_nep297_events_new_standard_event_account_version
    ON log_nep297_events_new (event_standard, event_event, account_id, event_version, id DESC);

ALTER TABLE log_nep297_events_new SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'event_standard,event_event,account_id,event_version',
    timescaledb.compress_orderby = 'id DESC'
);

SELECT add_compression_policy('log_nep297_events_new', BIGINT '250000000');

INSERT INTO log_nep297_events_new (
    id, block_height, block_timestamp, transaction_id, receipt_id,
    account_id, predecessor_id, event_standard, event_version,
    event_event, event_data
)
SELECT
    id, block_height, block_timestamp, transaction_id, receipt_id,
    account_id, predecessor_id, event_standard, event_version,
    event_event, event_data
FROM log_nep297_events;

SELECT setval(
    pg_get_serial_sequence('log_nep297_events_new', 'id'),
    (SELECT max(id) FROM log_nep297_events),
    true
);

ALTER TABLE log_nep297_events RENAME TO log_nep297_events_old;
ALTER INDEX idx_log_nep297_events_standard_event
    RENAME TO idx_log_nep297_events_old_standard_event;
ALTER INDEX idx_log_nep297_events_standard_event_account
    RENAME TO idx_log_nep297_events_old_standard_event_account;
ALTER INDEX idx_log_nep297_events_standard_event_account_version
    RENAME TO idx_log_nep297_events_old_standard_event_account_version;

ALTER TABLE log_nep297_events_new RENAME TO log_nep297_events;
ALTER INDEX idx_log_nep297_events_new_standard_event
    RENAME TO idx_log_nep297_events_standard_event;
ALTER INDEX idx_log_nep297_events_new_standard_event_account
    RENAME TO idx_log_nep297_events_standard_event_account;
ALTER INDEX idx_log_nep297_events_new_standard_event_account_version
    RENAME TO idx_log_nep297_events_standard_event_account_version;

-- Most of the events are ft_transfer, so postgres does a partial bitmap
-- scan or sequential scan, need to speed this up
CREATE INDEX idx_nep141_ft_transfer_partial ON log_nep297_events (id DESC)
WHERE event_standard = 'nep141' AND event_event = 'ft_transfer';

SELECT compress_chunk(show_chunks('log_nep297_events'));
