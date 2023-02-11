ALTER TABLE transactions
    ALTER COLUMN entry_account_code TYPE TEXT USING entry_account_code::TEXT,
    ALTER COLUMN exit_account_code TYPE TEXT USING exit_account_code::TEXT;