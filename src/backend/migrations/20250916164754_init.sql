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

CREATE TABLE "availability"
(
    id                SERIAL PRIMARY KEY,
    teacher_id        INTEGER REFERENCES teacher (id) ON DELETE CASCADE,
    day               day                                           NOT NULL,
    -- Time without seconds
    time              TIME(0) CHECK (EXTRACT(SECOND FROM time) = 0) NOT NULL,
    availability_type availability_type                             NOT NULL
);