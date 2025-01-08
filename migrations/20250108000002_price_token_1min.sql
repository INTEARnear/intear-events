-- no-transaction

CREATE MATERIALIZED VIEW price_token_1min_ohlc
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 minute', timestamp) AS bucket,
    token,
    FIRST(price_usd, timestamp) AS open,
    MAX(price_usd) AS high,
    MIN(price_usd) AS low,
    LAST(price_usd, timestamp) AS close
FROM price_token_events
GROUP BY bucket, token;
