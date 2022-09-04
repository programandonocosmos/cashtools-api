CREATE TABLE transactions (
    id UUID PRIMARY KEY,
    related_user UUID NOT NULL,
    entry_date DATE NOT NULL,
    entry_account_code TEXT,
    exit_account_code TEXT,
    amount FLOAT NOT NULL,
    description TEXT
);