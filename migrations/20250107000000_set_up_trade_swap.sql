CREATE EXTENSION timescaledb;

CREATE TABLE trade_swap_events (
    id BIGSERIAL,
    balance_changes JSONB NOT NULL,
    trader TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    receipt_id TEXT NOT NULL,
    block_height BIGINT NOT NULL,
    block_timestamp_nanosec TIMESTAMPTZ NOT NULL
);

SELECT create_hypertable('trade_swap_events', 'block_timestamp_nanosec', 
    chunk_time_interval => INTERVAL '1 day');

CREATE INDEX idx_trade_swap_events_balance_changes 
    ON trade_swap_events USING GIN (balance_changes jsonb_path_ops);

CREATE INDEX idx_trade_swap_events_trader
    ON trade_swap_events (trader, block_timestamp_nanosec DESC);

CREATE INDEX idx_trade_swap_events_txid
    ON trade_swap_events (transaction_id);
