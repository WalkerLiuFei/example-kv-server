use std::fs;
use lazy_static::lazy_static;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct APPConfig {
    pub redis_config: RedisConfig,
    pub host: String,
    pub port: i32,
    pub name: String,
    pub log_level: String,
    pub jaeger_endpoint: Option<String>,
}

#[derive(Debug, Deserialize,Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}



impl APPConfig {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(file_path)?;
        let config: APPConfig = toml::from_str(&config_str)?;
        Ok(config)
    }
}

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: APPConfig = {
         APPConfig::from_file("config.toml").expect("Failed to load the configuration.")
    };
}


#[cfg(test)]
mod tests {
    use lazy_static::initialize;
    use super::*;

    #[test]
    fn test_config() {
        let _ = initialize(&CONFIG);

        // Now you can access the CONFIG lazy-static variable in the test function.
        println!("{:?}", *CONFIG);

    }
}
