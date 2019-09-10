use serde::{Serialize, Deserialize};
use std::fs;
use std::path;
use std::io::{Read, Write, stdout};

use crate::error::*;

const HOST: &str = "https://circleci.com";
const ENDPOINT: &str = "gh/navikt";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub host: String,
    pub endpoint: String,
    pub token: String
}

impl Config {
    pub fn new(token: String) -> Config {
        Config {
            host: HOST.to_string(),
            endpoint: ENDPOINT.to_string(),
            token: token
        }
    }

    pub fn from_yaml_file<R: Read>(file: &mut R) -> DeployResult<Config> {
        let mut s = String::new();
        let _ = file.read_to_string(&mut s)?;
        let decoded = serde_yaml::from_str(&s)?;
        Ok(decoded)
    }

    pub fn to_yaml_file<W: Write>(&self, file: &mut W) -> DeployResult {
        let s = serde_yaml::to_vec(self)?;
        file.write_all(&s)?;
        Ok(())
    }

    pub fn get_dir() -> DeployResult<path::PathBuf> {
        let mut dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => panic!("Home dir does not exist?!"),
        };
        dir.push(".circleci");
        if !dir.exists() {
            let _ = fs::create_dir(&dir)?;
        }
        Ok(dir)
    }

    pub fn get_config() -> DeployResult<Config> {
        let mut f = Config::get_dir()?;
        f.push("cli-rs.yml");
        let mut config: Config;

        if !f.exists() {
            let mut file = fs::File::create(&f)?;
            println!("Config file does not exist, creating a new one.");
            println!("Please create a new token on CircleCI.");
            print!("Token: ");
            stdout().flush()?;
            webbrowser::open("https://circleci.com/account/api")?;
            let token: String = read!("{}\n");
            config = Config::new(token);
            config.to_yaml_file(&mut file)?;
        } else {
            let mut file = fs::File::open(&f)?;
            config = Config::from_yaml_file(&mut file)?;
        }
        Ok(config)
    }
}
