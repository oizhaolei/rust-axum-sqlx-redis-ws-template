create table users
(
    id            SERIAL PRIMARY KEY,
    username      text        not null,
    password_hash text        not null
);
