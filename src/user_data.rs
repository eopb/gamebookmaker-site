use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Deserialize, Debug, Serialize, Clone)]
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
    fn add_project(&mut self, name: &str) -> Self {
        self.projects.push(name.to_string());
        self.clone()
    }
    pub fn add_project_for_user(user: &str, name: &str) -> Result<(), std::io::Error> {
        let path = format!("data/{}/user_info.json", user);
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        drop(file);
        contents = serde_json::to_string(
            &serde_json::from_str::<Self>(&contents)
                .unwrap()
                .add_project(name),
        )
        .unwrap();
        let mut file = File::create(&path)?;
        file.write(contents.as_bytes())?;
        Ok(())
    }
}
