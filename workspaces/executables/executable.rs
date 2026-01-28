use std::collections::HashMap;

#[derive(Clone)]
pub struct Executable {
    pub name: String,
    pub description: String,
    pub args: HashMap<String, String>,
}
