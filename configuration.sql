CREATE TABLE CONFIGURATION (
    REPO_PATH TEXT NOT NULL
);

CREATE TABLE BRANCHES (
    NAME TEXT PRIMARY KEY,
    LATEST_KNOWN_COMMIT TEXT,
    DESCRIPTION TEXT
);

CREATE TABLE BUILD_DEFINITIONS (
    NAME TEXT PRIMARY KEY,
    DESCRIPTION TEXT
);

CREATE TABLE STEPS (
    ID INT PRIMARY KEY,
    NAME TEXT NOT NULL,
    BUILD_NAME TEXT REFERENCES BUILDS(BUILD_NAME),
    STEP_ORDER INT NOT NULL,
    DESCRIPTION TEXT,
    COMMAND TEXT NOT NULL,
    MAY_FAIL BOOLEAN DEFAULT('FALSE')
);

CREATE TABLE BUILDS (
    NAME TEXT PRIMARY KEY,
    DESCRIPTION TEXT,
    BRANCH_NAME TEXT REFERENCES BRANCHES(BRANCH_NAME),
    BUILD_DEFINITION REFERENCES BUILD_DEFINITIONS(NAME),
    ENABLED BOOLEAN DEFAULT('TRUE')
);

-------------------------------------------------------------------------------
-- Yalci default configuration
-------------------------------------------------------------------------------

INSERT INTO CONFIGURATION VALUES (
    '/home/fuszenecker/dev/Yalci'
);

INSERT INTO BRANCHES VALUES (
    'master',
    NULL,
    'Master branch of Yalci'
);

INSERT INTO BUILD_DEFINITIONS VALUES (
    'yalci master',
    'Build definition of Yalci master'
);

INSERT INTO STEPS VALUES
    (1, 'build', 'yalci master', 100, 'Building with Cargo', 'cargo build --release', 'false'),
    (2, 'test', 'yalci master', 200, 'Testing with Cargo', 'cargo test --release', 'false');