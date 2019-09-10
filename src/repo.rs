use git2::Repository;
use crate::error::*;


pub struct Repo {
    pub name: String,
    pub branch: String
}

impl Repo {
    pub fn new(name: String, branch: String) -> Repo {
        Repo {
            name: name,
            branch: branch
        }
    }

    pub fn from_env() -> DeployResult<Repo> {
        let repo = Repository::open_from_env()?;
        let head = repo.head()?;
        let origin = repo.find_remote("origin")?;
        let url = match origin.url() {
            Some(giturl) => giturl.split("/").last().unwrap(), // TODO: unwrap >:(
            None => return Err(DeployError::RepoError("Cannot find URL for origin".to_string())),
        };

        let name = match url.split(".git").next() {
            Some(name) => name,
            None => return Err(DeployError::RepoError("Malformed remote URL: does not end with .git".to_string())),
        };
        let branch = match head.name() {
            Some(branch) => branch.split("/").last().unwrap(), // TODO: unwrap >:(
            None => return Err(DeployError::RepoError("Could not get name for branch on HEAD".to_string()))
        };
        Ok(Repo {
            name: name.to_string(),
            branch: branch.to_string()
        })
    }

}
