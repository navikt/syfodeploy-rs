//! Error types and conversion functions.

use std;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum DeployError {
    ConfigError(String),
    RepoError(String),
    IOError(Arc<std::io::Error>),
}

impl fmt::Display for DeployError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeployError::ConfigError(ref s) => write!(f, "Config error: {}", s),
            _ => write!(f, "DeployError {:?}", self),
        }
    }
}

impl Error for DeployError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            DeployError::IOError(ref e) => Some(&**e),
            _ => None,
        }
    }
}

/// A convenient result type consisting of a return type and a `DeployError`
pub type DeployResult<T = ()> = Result<T, DeployError>;

impl From<std::io::Error> for DeployError {
    fn from(e: std::io::Error) -> DeployError {
        DeployError::IOError(Arc::new(e))
    }
}

impl From<serde_yaml::Error> for DeployError {
    fn from(e: serde_yaml::Error) -> DeployError {
        let errstr = format!("YAML error: {}", e.description());

        DeployError::ConfigError(errstr)
    }
}

impl From<git2::Error> for DeployError {
    fn from(e: git2::Error) -> DeployError {
        let errstr = format!("Repo error: {}", e.description());

        DeployError::RepoError(errstr)
    }
}
