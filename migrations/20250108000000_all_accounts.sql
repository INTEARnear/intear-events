CREATE TABLE all_accounts (
    account_id varchar(64) NOT NULL PRIMARY KEY
);

CREATE INDEX idx_all_accounts_prefix ON all_accounts USING btree (account_id text_pattern_ops);
CREATE INDEX idx_all_accounts_suffix ON all_accounts USING btree (reverse(account_id) text_pattern_ops);
