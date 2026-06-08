CREATE TABLE IF NOT EXISTS containers (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    name      TEXT    NOT NULL,
    parent_id INTEGER REFERENCES containers(id)
);

CREATE TABLE IF NOT EXISTS minis (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    name           TEXT    NOT NULL,
    number_printed INTEGER NOT NULL DEFAULT 0,
    base_size      INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS terrains (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    name           TEXT    NOT NULL,
    number_printed INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS mini_locations (
    mini_id      INTEGER NOT NULL REFERENCES minis(id),
    container_id INTEGER NOT NULL REFERENCES containers(id),
    PRIMARY KEY (mini_id, container_id)
);

CREATE TABLE IF NOT EXISTS terrain_locations (
    terrain_id   INTEGER NOT NULL REFERENCES terrains(id),
    container_id INTEGER NOT NULL REFERENCES containers(id),
    PRIMARY KEY (terrain_id, container_id)
);
