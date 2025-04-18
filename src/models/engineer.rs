use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Engineer {
    pub name: String,
}

impl Engineer {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}