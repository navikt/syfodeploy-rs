#[macro_use] extern crate text_io;
extern crate reqwest;
extern crate webbrowser;
extern crate dirs;
extern crate serde;
extern crate serde_yaml;
extern crate git2;

mod repo;
mod error;
mod config;
mod payload;

use config::Config;
use payload::Payload;
use repo::Repo;
use std::env;


fn main() {
    let config = Config::get_config().unwrap();
    let args: Vec<String> = env::args().collect();
    let mut r: Repo;
    if args.len() >= 3 {
        r = Repo::new(args[1].to_owned(), args[2].to_owned());
    } else {
        r = match Repo::from_env() {
            Ok(r) => r,
            Err(e) => panic!("{:?}", e),
        };
    }
    let client = reqwest::Client::new();
    let payload = Payload::new(r.branch);
    let res = client.post(&format!("{}/api/v2/project/{}/{}/pipeline?circle-token={}", config.host, config.endpoint, r.name, config.token))
        .json(&payload)
        .send()
        .unwrap(); // TODO: unwrap >:(
    // let _json: Response = res.json().unwrap(); // TODO: might be needed? also, unwrap >:(
    if res.status().is_success() {
        webbrowser::open(&format!("{}/{}/{}", config.host, config.endpoint, r.name)).unwrap(); // TODO: unwrap >:(
    } else {
        panic!(format!("The request failed somehow: {:?}", res))
    }
}
