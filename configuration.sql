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
    MAY_FAIL BOOLEAN DEFAULT('FALSE'),
    ENABLED BOOLEAN DEFAULT('TRUE')
);

CREATE TABLE BUILDS (
    BRANCH_NAME TEXT REFERENCES BRANCHES(NAME),
    BUILD_DEFINITION REFERENCES BUILD_DEFINITIONS(NAME),
    ENABLED BOOLEAN DEFAULT('TRUE')
);

-------------------------------------------------------------------------------
-- Yalci default configuration
-------------------------------------------------------------------------------

INSERT INTO CONFIGURATION VALUES (
    '/home/fuszenecker/dev/Yalci'
);

INSERT INTO BRANCHES VALUES 
    ('master', NULL, 'Master branch of Yalci');

INSERT INTO BUILD_DEFINITIONS VALUES 
    ('CI build', 'Continuous integration build definition');

INSERT INTO STEPS VALUES
    (1, 'build', 'CI build', 100, 'Building with Cargo', 'cargo build --release', 'FALSE', 'TRUE'),
    (2, 'test',  'CI build', 200, 'Testing with Cargo', 'cargo test --release', 'FALSE', 'TRUE');

INSERT INTO BUILDS VALUES 
    ('master', 'CI build', 'TRUE');