use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    created_at: SystemTime,
    updated_at: SystemTime,
}

impl Metadata {
    pub fn new(created_at: SystemTime, updated_at: SystemTime) -> Self {
        Metadata {
            created_at,
            updated_at,
        }
    }
}
