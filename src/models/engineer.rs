use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Engineer {
    pub name: String,
}

impl Engineer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
