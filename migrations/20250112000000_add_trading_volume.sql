CREATE TABLE trading_volume_by_minute (
    token_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    volume NUMERIC NOT NULL DEFAULT 0,
    PRIMARY KEY (token_id, time)
);

SELECT create_hypertable('trading_volume_by_minute', 'time');

CREATE INDEX idx_trading_volume_token_time ON trading_volume_by_minute (token_id, time DESC);
