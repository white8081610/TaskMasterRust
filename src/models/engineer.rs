use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Engineer {
    pub name: String,
}

impl Engineer {
    pub fn new(name: String) -> Self {
        Engineer { name }
    }
}

impl PartialEq for Engineer {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}