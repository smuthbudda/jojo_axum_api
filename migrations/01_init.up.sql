CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY NOT NULL,
    text VARCHAR(255) NOT NULL,
    done BOOLEAN default false NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_name VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    phone VARCHAR,
    active BOOLEAN,
    password VARCHAR
);

CREATE TABLE IF NOT EXISTS points (
    id SERIAL PRIMARY KEY,
    points INTEGER,
    gender VARCHAR(10),
    category VARCHAR(20),
    event VARCHAR(20),
    mark FLOAT,
    mark_time TIME DEFAULT '00:00:00'
);