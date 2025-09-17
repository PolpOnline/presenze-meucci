-- Add migration script here
CREATE TABLE "user"
(
    id       SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

CREATE TYPE day AS ENUM ('Lun', 'Mar', 'Mer', 'Gio', 'Ven', 'Sab', 'Dom');

CREATE TABLE "teacher"
(
    id        SERIAL PRIMARY KEY,
    full_name TEXT NOT NULL
);

CREATE TABLE "availability"
(
    id         SERIAL PRIMARY KEY,
    teacher_id INTEGER REFERENCES teacher (id) ON DELETE CASCADE,
    day        day                                           NOT NULL,
    -- Time without seconds
    time       TIME(0) CHECK (EXTRACT(SECOND FROM time) = 0) NOT NULL
);