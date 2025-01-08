CREATE TABLE newtoken_nep141_events (
    id BIGSERIAL PRIMARY KEY,
    account_id TEXT NOT NULL UNIQUE,
    transaction_id TEXT NOT NULL,
    receipt_id TEXT NOT NULL,
    block_height BIGINT NOT NULL,
    block_timestamp_nanosec TIMESTAMPTZ NOT NULL
);
