CREATE TABLE IF NOT EXISTS users (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    google_id VARCHAR(255) UNIQUE,
    username VARCHAR(100) UNIQUE NOT NULL,
    first_name VARCHAR(748) NOT NULL,
    last_name VARCHAR(748) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() at time zone('utc')) NOT NULL
);