use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    latitude: f32,
    longitude: f32,
}

impl Location {
    pub fn new(latitude: f32, longitude: f32) -> Self {
        Location {
            latitude,
            longitude,
        }
    }
}
