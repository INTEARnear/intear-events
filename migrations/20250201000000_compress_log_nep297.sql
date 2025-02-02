ALTER TABLE log_nep297_events SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'event_standard,event_event,account_id,event_version',
    timescaledb.compress_orderby = 'id DESC'
);

SELECT add_compression_policy('log_nep297_events', BIGINT '100000000');

-- Compress all existing chunks
SELECT compress_chunk(c) FROM show_chunks('log_nep297_events') c;
