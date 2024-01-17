use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    id: u32,
    name: String,
}

impl Label {
    pub fn new(id: u32, name: String) -> Self {
        Label { id, name }
    }
}
