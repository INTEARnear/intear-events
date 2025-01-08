CREATE TABLE price_token_events (
    id BIGSERIAL,
    token TEXT NOT NULL,
    price_usd NUMERIC NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);

SELECT create_hypertable('price_token_events', 'timestamp', 
    chunk_time_interval => INTERVAL '8 hours');

CREATE INDEX idx_price_token_events_token_time 
    ON price_token_events (token, timestamp DESC);
