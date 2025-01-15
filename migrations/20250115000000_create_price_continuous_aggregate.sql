-- no-transaction
CREATE MATERIALIZED VIEW price_minutes_mv
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 minute', timestamp) AS bucket,
    token,
    last(price_usd, timestamp) AS last_price
FROM price_token_events
GROUP BY 1, 2;
