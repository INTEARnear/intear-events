CREATE TABLE tx_transaction_events (
    id BIGSERIAL,
    block_timestamp TIMESTAMPTZ NOT NULL,
    block_height BIGINT NOT NULL,
    signer_id TEXT NOT NULL,
    public_key TEXT NOT NULL,
    nonce BIGINT NOT NULL,
    receiver_id TEXT NOT NULL,
    priority_fee BIGINT,
    signature TEXT NOT NULL,
    transaction_id TEXT NOT NULL
);

SELECT create_hypertable('tx_transaction_events', 'block_timestamp', 
    chunk_time_interval => INTERVAL '4 hours');

CREATE INDEX idx_tx_transaction_events_txid
    ON tx_transaction_events (transaction_id, block_timestamp DESC);

CREATE INDEX idx_tx_transaction_events_signer
    ON tx_transaction_events (signer_id, block_timestamp DESC);

CREATE INDEX idx_tx_transaction_events_receiver
    ON tx_transaction_events (receiver_id, block_timestamp DESC);
