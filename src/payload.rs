use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Response {
    pub number: usize,
    pub state: String,
    pub id: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    pub deploy_branch: bool
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub branch: String,
    pub parameters: Params,
}

impl Payload {
    pub fn new(branch: String) -> Payload {
        Payload {
            branch: branch,
            parameters: Params {
                deploy_branch: true
            }
        }
    }
}
