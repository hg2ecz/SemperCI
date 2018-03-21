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
    pub may_fail: Option<bool>
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

                // Loading branches
                let mut stmt = conn.prepare("SELECT NAME, LATEST_KNOWN_COMMIT, DESCRIPTION FROM BRANCHES")?;

                let mut branch_rows = stmt.query_map(&[], |row| {
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

                Ok (configuration)
            }
            _ => Err(ConfigurationError::MissingTable("CONFIGURATION".to_string()))
        }
    }
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

    #[test]
    fn check_repo_path() {
        let configuration = Configuration::new("configuration.db").unwrap();
        assert_eq!("/home/fuszenecker/dev/Yalci", configuration.repo_path);
    }

    #[test]
    fn check_master_branch() {
        let configuration = Configuration::new("configuration.db").unwrap();
        assert_eq!("master", configuration.branches[0].name);
    }}