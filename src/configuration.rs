use std::vec::Vec;
use rusqlite::Connection;
use rusqlite::Error as RusqError;

pub struct Configuration {
    pub repo_path: String,
    pub branches: Vec<Branch>
}

#[derive(Debug)]
pub struct Branch {
    pub name: String,
    pub last_known_commit: Option<String>,
    pub description: Option<String>,
    pub build_definitions: Vec<BuildDefinition>
}

#[derive(Debug)]
pub struct BuildDefinition {
    pub name: String,
    pub description: Option<String>,
    pub steps: Vec<Step>
}

#[derive(Debug)]
pub struct Step {
    pub name: String,
    pub description: Option<String>,
    pub command: String,
    pub rollback_command: Option<String>,
    pub may_fail: bool
}

impl Configuration {
    pub fn new(path: &str) -> Result<Configuration, ConfigurationError> {
        let conn = Connection::open(path)?;

        // Loading repo path
        let mut stmt = conn.prepare("SELECT REPO_PATH FROM CONFIGURATION")?;

        let mut configuration_row = stmt.query_map(&[], |row| {
            Configuration {
                repo_path: row.get(0),
                branches: Vec::new()
            }
        })?;

        match configuration_row.next() {
            Some (value) => {
                let mut configuration = value?;
                add_branches(&conn, &mut configuration)?;

                //let _build_definitions = load_build_definitions(&conn)?;
                Ok (configuration)
            }
            _ => Err(ConfigurationError::MissingTable("CONFIGURATION".to_string()))
        }
    }
}

fn add_branches(conn: &Connection, configuration: &mut Configuration) -> Result<(), ConfigurationError> {
    // Loading branches
    let mut stmt = conn.prepare("SELECT NAME, LATEST_KNOWN_COMMIT, DESCRIPTION FROM BRANCHES")?;

    let branch_rows = stmt.query_map(&[], |row| {
        Branch {
            name: row.get(0),
            last_known_commit: row.get(1),
            description: row.get(2),
            build_definitions: Vec::new()
        }
    })?;

    for branch in branch_rows {
        debug!("Adding branch: {:?}", &branch);
        configuration.branches.push(branch?);
    }

    Ok(())
}

#[allow(dead_code)]
fn load_build_definitions(conn: &Connection) -> Result<Vec<BuildDefinition>, ConfigurationError> {
    let mut stmt = conn.prepare("SELECT NAME, DESCRIPTION FROM BUILD_DEFINITIONS")?;

    let definition_rows = stmt.query_map(&[], |row| {
        BuildDefinition {
            name: row.get(0),
            description: row.get(1),
            steps: Vec::new()
        }
    })?;

    let mut build_definitions = Vec::new();

    for build_definition in definition_rows {
        debug!("Loading build definition: {:?}", &build_definition);

        let mut definition = build_definition?;

        definition.steps = load_steps(conn, &definition.name)?;

        build_definitions.push(definition);
    }

    Ok(build_definitions)
}

#[allow(dead_code)]
pub fn load_steps(conn: &Connection, build_definition_name: &String) -> Result<Vec<Step>, ConfigurationError> {
    let mut stmt = conn.prepare("SELECT 
        NAME, DESCRIPTION,
        COMMAND, ROLLBACK_COMMAND, MAY_FAIL
        FROM STEPS
        WHERE BUILD_NAME = ?1
            AND ENABLED = 'TRUE'
        ORDER BY STEP_ORDER")?;

    let step_rows = stmt.query_map(&[build_definition_name], |row| {
        let may_fail: String = row.get(4);

        Step {
            name: row.get(0),
            description: row.get(1),
            command: row.get(2),
            rollback_command: row.get(3),
            may_fail: may_fail.to_uppercase() == "TRUE"
        }
    })?;

    let mut steps = Vec::new();

    for step in step_rows {
        debug!("Loading step: {:?}", &step);
        steps.push(step?);
    }

    Ok(steps)
}

#[derive(Debug)]
pub enum ConfigurationError {
    NotFound(String),
    MissingTable(String),
    GenericError
}

impl From<RusqError> for ConfigurationError {
    fn from(error: RusqError) -> ConfigurationError {
        match error {
            RusqError::InvalidPath(path) => {
                match path.to_str() {
                    Some (string) => ConfigurationError::NotFound(String::from(string)),
                    _ => ConfigurationError::NotFound(String::from("NO PATH"))
                }
            },
            _ => ConfigurationError::GenericError 
        }
    }
}

#[cfg(test)]
mod configuration_tests {
    use Configuration;
    use rusqlite::Connection;

    #[test]
    fn check_repo_path() {
        let configuration = Configuration::new("configuration.db").unwrap();
        assert_eq!("/home/fuszenecker/dev/SemperCI", configuration.repo_path);
    }

    #[test]
    fn check_master_branch() {
        let configuration = Configuration::new("configuration.db").unwrap();
        assert_eq!("master", configuration.branches[0].name);
    }

    use configuration::load_steps;

    #[test]
    fn load_build_steps_by_build_name() {
        let conn = Connection::open("configuration.db").unwrap();
        let steps = load_steps(&conn, &String::from("CI build")).unwrap();       
        assert!(!steps.is_empty());
        assert_eq!(2, steps.len());
        assert_eq!(String::from("test"), steps[1].name);

        match steps[1].description {
            Some (ref d) =>
                assert_eq!(String::from("Testing with Cargo"), *d),
            _ => assert!(false)
        }

        assert_eq!(String::from("cargo test --release"), steps[1].command);
        assert!(steps[1].rollback_command.is_none());
        assert!(!steps[1].may_fail);
    }
}