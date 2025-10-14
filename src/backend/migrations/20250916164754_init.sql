CREATE TABLE "user"
(
    id       SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

CREATE TABLE import
(
    id        SERIAL PRIMARY KEY,
    user_id   INTEGER REFERENCES "user" (id) ON DELETE CASCADE NOT NULL,
    import_ts TIMESTAMP                                        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    file_name TEXT                                             NOT NULL,
    begin_ts  TIMESTAMP                                        NOT NULL,
    end_ts    TIMESTAMP                                        NOT NULL CHECK (end_ts >= begin_ts)
);

-- ISO day-of-week domain: 1=Mon .. 7=Sun
CREATE DOMAIN isodow AS SMALLINT CHECK (VALUE BETWEEN 1 AND 7);

CREATE TABLE "teacher"
(
    id        SERIAL PRIMARY KEY,
    full_name TEXT                                             NOT NULL,
    import_id INTEGER REFERENCES import (id) ON DELETE CASCADE NOT NULL
);

-- Recupero Orario or Availability, translated in English
CREATE TYPE availability_type AS ENUM ('Availability', 'RecoveryHours');

-- Time without seconds
CREATE DOMAIN time_no_seconds AS TIME(0)
    CHECK (EXTRACT(SECOND FROM VALUE) = 0);

CREATE TABLE "availability"
(
    id                SERIAL PRIMARY KEY,
    teacher_id        INTEGER REFERENCES teacher (id) ON DELETE CASCADE NOT NULL,
    day               isodow                                            NOT NULL,
    time              time_no_seconds                                   NOT NULL,
    availability_type availability_type                                 NOT NULL
);

CREATE TABLE room
(
    id        SERIAL PRIMARY KEY,
    name      TEXT,
    import_id INTEGER REFERENCES import (id) ON DELETE CASCADE NOT NULL
);

CREATE TABLE "group"
(
    id        SERIAL PRIMARY KEY,
    name      TEXT                                             NOT NULL,
    import_id INTEGER REFERENCES import (id) ON DELETE CASCADE NOT NULL
);

CREATE TABLE lesson
(
    id         SERIAL PRIMARY KEY,
    teacher_id INTEGER REFERENCES teacher (id) ON DELETE CASCADE NOT NULL,
    day        isodow                                            NOT NULL,
    time       time_no_seconds                                   NOT NULL,
    room_id    INTEGER REFERENCES room (id) ON DELETE CASCADE,
    group_id   INTEGER REFERENCES "group" (id) ON DELETE CASCADE,
    duration   SMALLINT CHECK (duration > 0)                     NOT NULL DEFAULT 1
);

CREATE TYPE absence_status AS ENUM ('Uncovered', 'ClassDelayed', 'ClassCanceled', 'SubstituteFound');

CREATE TABLE "absence"
(
    id                              SERIAL PRIMARY KEY,
    absent_teacher_lesson           INTEGER REFERENCES lesson (id) ON DELETE CASCADE NOT NULL,
    absence_date                    DATE                                             NOT NULL DEFAULT CURRENT_DATE,
    status                          absence_status                                   NOT NULL DEFAULT 'Uncovered',
    substitute_teacher_availability INTEGER REFERENCES availability (id) ON DELETE CASCADE
        -- if substitute_teacher_availability is set, status must be 'SubstituteFound'
        CONSTRAINT absence_substitute_status_check
            CHECK (
                (substitute_teacher_availability IS NOT NULL AND status = 'SubstituteFound')
                    OR
                (substitute_teacher_availability IS NULL AND status <> 'SubstituteFound')
                )
);