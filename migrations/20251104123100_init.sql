CREATE TABLE IF NOT EXISTS user(
    id                          INTEGER     PRIMARY KEY,
    username                    TEXT        NOT NULL,
    password                    TEXT        NOT NULL,
    new_feature_notifications   INTEGER     NOT NULL,
    location_history            INTEGER     NOT NULL,
    heartbeat_history           INTEGER     NOT NULL,

    UNIQUE(username)
);

CREATE TABLE IF NOT EXISTS partner(
    id          INTEGER     PRIMARY KEY,
    user_id_1   INTEGER     NOT NULL,
    user_id_2   INTEGER     NOT NULL,

    FOREIGN KEY(user_id_1) REFERENCES user(id),
    FOREIGN KEY(user_id_2) REFERENCES user(id)
);

CREATE TABLE IF NOT EXISTS question(
    id              INTEGER     PRIMARY KEY,
    category        INTEGER     NOT NULL,
    prompt          TEXT        NOT NULL,
    response_type   BLOB        NOT NULL
);

CREATE TABLE IF NOT EXISTS answer(
    id              INTEGER     PRIMARY KEY,
    question_id     INTEGER     NOT NULL,
    user_id         INTEGER     NOT NULL,
    timestamp       TEXT        NOT NULL,
    response        BLOB        NOT NULL,

    FOREIGN KEY(question_id) REFERENCES question(id),
    FOREIGN KEY(user_id) REFERENCES user(id)
);

CREATE TABLE IF NOT EXISTS location(
    id              INTEGER     PRIMARY KEY,
    user_id         INTEGER     NOT NULL,
    timestamp       TEXT        NOT NULL,
    latitude        REAL        NOT NULL,
    longitude       REAL        NOT NULL,
    accuracy        INTEGER     NOT NULL,

    FOREIGN KEY(user_id) REFERENCES user(id)
);

CREATE TABLE IF NOT EXISTS heartbeat(
    id              INTEGER     PRIMARY KEY,
    user_id         INTEGER     NOT NULL,
    start_timestamp TEXT        NOT NULL,
    end_timestamp   TEXT,       -- NULLABLE

    FOREIGN KEY(user_id) REFERENCES user(id)
);
