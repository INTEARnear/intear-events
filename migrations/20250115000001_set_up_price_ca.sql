CREATE INDEX ON price_minutes_mv (token, bucket DESC);

SELECT add_continuous_aggregate_policy('price_minutes_mv',
    start_offset => INTERVAL '5 minutes',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute');
