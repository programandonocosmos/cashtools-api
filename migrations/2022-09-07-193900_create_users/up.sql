CREATE EXTENSION "uuid-ossp";

CREATE TABLE users (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    register_date TIMESTAMP,
    email TEXT UNIQUE NOT NULL,
    last_code_gen_request TIMESTAMP,
    login_code INTEGER
);