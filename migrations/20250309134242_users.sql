CREATE TABLE users (
    id SERIAL PRIMARY KEY,

    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,

    first_name VARCHAR(255),
    second_name VARCHAR(255),
    birth_date DATE,
    gender VARCHAR(255),
    biography VARCHAR(255),
    city VARCHAR(255)
);
