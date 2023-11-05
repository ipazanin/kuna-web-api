use std::env::{self};

use serde::Deserialize;

use super::{config_section::ConfigSection, server_config::ServerConfig};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename(deserialize = "serverConfig"))]
    server_config: ServerConfig,
}

impl Config {
    pub fn server_config(&self) -> &ServerConfig {
        &self.server_config
    }

    pub fn set_from_environment(&mut self) {
        let environment_variables = env::vars();
        for (name, value) in environment_variables {
            let name_segments: Vec<&str> = name.split("__").collect();

            if name_segments.len() == 1 {
                continue;
            }

            let first_segment = name_segments[0];
            let ending_of_first_segment = first_segment.len() + 2;

            if name.len() <= ending_of_first_segment {
                continue;
            }

            let sub_field_name = &name[ending_of_first_segment..];

            if let "serverconfig" = first_segment.to_lowercase().as_str() {
                self.server_config
                    .set_from_environment(sub_field_name, value.as_str())
            }
        }
    }
}
