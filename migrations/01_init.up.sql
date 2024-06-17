CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY NOT NULL,
    text VARCHAR(255) NOT NULL,
    done BOOLEAN DEFAULT false NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_name VARCHAR(255) NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(20),
    active BOOLEAN DEFAULT false,
    password VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS points (
    id SERIAL PRIMARY KEY,
    points INTEGER,
    gender VARCHAR(10),
    category VARCHAR(20),
    event VARCHAR(20),
    mark FLOAT
);

CREATE TABLE IF NOT EXISTS user_pages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS page_template (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_page_id UUID REFERENCES user_pages(id) NOT NULL,
    page_title VARCHAR(100),
    page_body VARCHAR(10000)
);
