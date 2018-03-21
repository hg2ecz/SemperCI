use std::vec::Vec;
use rusqlite::Connection;
use rusqlite::Error as RusqError;

pub struct Configuration {
    pub repo_path: String,
    pub branches: Vec<Branches>
}

pub struct Branches {
    pub name: String,
    pub last_known_commit: String,
    pub description: String,
    pub build_definitions: Vec<BuildDefinitions>
}

pub struct BuildDefinitions {
    pub name: String,
    pub description: String,
    pub steps: Vec<Steps>
}

pub struct Steps {
    pub name: String,
    pub description: String,
    pub command: String,
    pub rollback_command: String,
    pub may_fail: Option<bool>
}

impl Configuration {
    pub fn new(path: &str) -> Result<Configuration, ConfigurationError> {
        let conn = Connection::open(path)?;

        let mut stmt = conn.prepare("SELECT REPO_PATH FROM CONFIGURATION")?;

        let mut configuration_row = stmt.query_map(&[], |row| {
            Configuration {
                repo_path: row.get(0),
                branches: Vec::new()
            }
        })?;

        match configuration_row.next() {
            Some (value) => Ok (value?),
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
}