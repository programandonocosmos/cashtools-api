ALTER TABLE transactions
    ALTER COLUMN entry_account_code TYPE UUID USING entry_account_code::UUID,
    ALTER COLUMN exit_account_code TYPE UUID USING exit_account_code::UUID;