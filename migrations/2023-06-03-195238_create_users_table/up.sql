CREATE TABLE IF NOT EXISTS users (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    username VARCHAR(100) UNIQUE,
    first_name VARCHAR(748),
    last_name VARCHAR(748),
    email VARCHAR(100) UNIQUE,
    password VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() at time zone('utc'))
);