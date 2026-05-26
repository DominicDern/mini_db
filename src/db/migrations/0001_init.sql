CREATE TABLE IF NOT EXISTS containers (
    id      INTEGER PRIMARY KEY,
    name    TEXT    NOT NULL,
    parent_id INTEGER REFERENCES containers(id)  -- NULL = top-level
);

CREATE TABLE IF NOT EXISTS minis (
    id             INTEGER PRIMARY KEY,
    name           TEXT    NOT NULL,
    number_printed INTEGER NOT NULL DEFAULT 0,
    base_size      INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS terrains (
    id             INTEGER PRIMARY KEY,
    name           TEXT    NOT NULL,
    number_printed INTEGER NOT NULL DEFAULT 0
);

-- Many-to-many: a mini can be in multiple containers
CREATE TABLE IF NOT EXISTS mini_locations (
    mini_id      INTEGER NOT NULL REFERENCES minis(id),
    container_id INTEGER NOT NULL REFERENCES containers(id),
    PRIMARY KEY (mini_id, container_id)
);

-- Many-to-many: a terrain can be in multiple containers
CREATE TABLE IF NOT EXISTS terrain_locations (
    terrain_id   INTEGER NOT NULL REFERENCES terrains(id),
    container_id INTEGER NOT NULL REFERENCES containers(id),
    PRIMARY KEY (terrain_id, container_id)
);
