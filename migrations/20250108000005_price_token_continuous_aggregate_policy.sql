SELECT add_continuous_aggregate_policy('price_token_1min_ohlc',
    start_offset => INTERVAL '5 minutes',
    end_offset => INTERVAL '1 second',
    schedule_interval => INTERVAL '1 second');

SELECT add_continuous_aggregate_policy('price_token_1hour_ohlc',
    start_offset => INTERVAL '3 hours',
    end_offset => INTERVAL '5 seconds',
    schedule_interval => INTERVAL '5 seconds');

SELECT add_continuous_aggregate_policy('price_token_1day_ohlc',
    start_offset => INTERVAL '3 days',
    end_offset => INTERVAL '10 seconds',
    schedule_interval => INTERVAL '10 seconds');
