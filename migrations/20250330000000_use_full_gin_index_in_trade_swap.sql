DROP INDEX idx_trade_swap_events_balance_changes;

CREATE INDEX idx_trade_swap_events_balance_changes
    ON trade_swap_events USING GIN (balance_changes);

ANALYZE trade_swap_events;
