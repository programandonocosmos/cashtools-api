CREATE TYPE earning_index_enum AS ENUM('CDI', 'FIXED', 'IPCA');

CREATE TABLE accounts (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    related_user UUID NOT NULL,
    time TIMESTAMP NOT NULL,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    last_calculated_balance FLOAT NOT NULL,
    is_pre_allocation BOOLEAN NOT NULL,
    pre_allocation_amount FLOAT,
    pre_allocation_accumulative BOOLEAN,
    is_earning BOOLEAN NOT NULL,
    earning_rate FLOAT,
    earning_index earning_index_enum,
    is_available BOOLEAN NOT NULL,
    in_trash BOOLEAN NOT NULL
);