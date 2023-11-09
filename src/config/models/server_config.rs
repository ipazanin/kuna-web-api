use serde::Deserialize;

use super::config_section::ConfigSection;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    port: u32,
}

impl ServerConfig {
    pub fn port(&self) -> u32 {
        self.port
    }
}

impl ConfigSection for ServerConfig {
    fn set_from_environment(&mut self, name: &str, value: &str) {
        if let "port" = name.to_lowercase().as_str() {
            let port: u32 = value
                .parse()
                .expect("Invalid value for port number in server config");
            self.port = port;
        }
    }
}
