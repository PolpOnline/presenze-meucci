-- Add migration script here
CREATE TABLE "user"
(
    id       SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

CREATE TYPE day AS ENUM ('Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun');

CREATE TABLE "teacher"
(
    id        SERIAL PRIMARY KEY,
    full_name TEXT NOT NULL
);

-- Recupero Orario or Availability, translated in English
CREATE TYPE availability_type AS ENUM ('Availability', 'RecoveryHours');

-- Time without seconds
CREATE DOMAIN time_no_seconds AS TIME(0)
    CHECK (EXTRACT(SECOND FROM VALUE) = 0);

CREATE TABLE "availability"
(
    id                SERIAL PRIMARY KEY,
    teacher_id        INTEGER           REFERENCES teacher (id) ON DELETE SET NULL,
    day               day               NOT NULL,
    time              time_no_seconds   NOT NULL,
    availability_type availability_type NOT NULL
);

CREATE TABLE room
(
    id   SERIAL PRIMARY KEY,
    name TEXT
);

CREATE TABLE lesson
(
    id         SERIAL PRIMARY KEY,
    teacher_id INTEGER REFERENCES teacher (id) ON DELETE CASCADE NOT NULL,
    day        day                                               NOT NULL,
    time       time_no_seconds                                   NOT NULL,
    room_id    INTEGER REFERENCES room (id) ON DELETE CASCADE
);

