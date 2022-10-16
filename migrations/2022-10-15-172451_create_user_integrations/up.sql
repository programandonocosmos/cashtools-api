CREATE TABLE user_integrations (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    related_user UUID NOT NULL,
    name TEXT NOT NULL,
    time TIMESTAMP NOT NULL
);