CREATE TABLE log_nep297_events (
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

SELECT create_hypertable('log_nep297_events', 'block_timestamp', 
    chunk_time_interval => INTERVAL '4 hours');

CREATE INDEX idx_log_nep297_events_standard_event 
    ON log_nep297_events (event_standard, event_event, block_timestamp DESC);

CREATE INDEX idx_log_nep297_events_standard_event_account 
    ON log_nep297_events (event_standard, event_event, account_id, block_timestamp DESC);

CREATE INDEX idx_log_nep297_events_standard_event_account_version 
    ON log_nep297_events (event_standard, event_event, account_id, event_version, block_timestamp DESC);
