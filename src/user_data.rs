use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug, Serialize)]
pub struct UserInfo {
    projects: Vec<String>,
}

impl Default for UserInfo {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
        }
    }
}

impl UserInfo {
    pub fn get(name: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(format!("data/{}/user_info.json", name))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_json::from_str(&contents).unwrap())
    }
    // Do not use
    pub fn exmaple() -> Self {
        Self {
            projects: vec![
                "project 1".to_string(),
                "project 2".to_string(),
                "project 3".to_string(),
            ],
        }
    }
}
