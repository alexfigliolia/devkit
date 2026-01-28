use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DevKitConfig {
    pub project: String,
    pub workspaces: Vec<String>,
    pub commands: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct DevKitCommand {
    pub name: String,
    pub location: String,
    pub description: String,
    pub commands: HashMap<String, String>,
}
