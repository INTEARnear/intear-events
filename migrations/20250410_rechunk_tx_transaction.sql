CREATE TABLE tx_transaction_events_new (
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

SELECT create_hypertable('tx_transaction_events_new', 'id',
    chunk_time_interval => 50000000);

CREATE INDEX idx_tx_transaction_events_new_txid
    ON tx_transaction_events_new (transaction_id, id DESC);
CREATE INDEX idx_tx_transaction_events_new_signer
    ON tx_transaction_events_new (signer_id, id DESC);
CREATE INDEX idx_tx_transaction_events_new_receiver
    ON tx_transaction_events_new (receiver_id, id DESC);

INSERT INTO tx_transaction_events_new (
    id, block_timestamp, block_height, signer_id, public_key,
    nonce, receiver_id, priority_fee, signature, transaction_id
) SELECT
    id, block_timestamp, block_height, signer_id, public_key,
    nonce, receiver_id, priority_fee, signature, transaction_id
FROM tx_transaction_events;

SELECT setval(
    pg_get_serial_sequence('tx_transaction_events_new', 'id'),
    (SELECT max(id) FROM tx_transaction_events),
    true
);

ALTER TABLE tx_transaction_events RENAME TO tx_transaction_events_old;
ALTER INDEX idx_tx_transaction_events_txid
    RENAME TO idx_tx_transaction_events_old_txid;
ALTER INDEX idx_tx_transaction_events_signer
    RENAME TO idx_tx_transaction_events_old_signer;
ALTER INDEX idx_tx_transaction_events_receiver
    RENAME TO idx_tx_transaction_events_old_receiver;

ALTER TABLE tx_transaction_events_new RENAME TO tx_transaction_events;
ALTER INDEX idx_tx_transaction_events_new_txid
    RENAME TO idx_tx_transaction_events_txid;
ALTER INDEX idx_tx_transaction_events_new_signer
    RENAME TO idx_tx_transaction_events_signer;
ALTER INDEX idx_tx_transaction_events_new_receiver
    RENAME TO idx_tx_transaction_events_receiver;
