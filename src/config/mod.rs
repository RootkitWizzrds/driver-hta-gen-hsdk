use std::fs;
use toml::Value;

pub fn load_config(path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(path)?;
    let config: Value = toml::de::from_str(&config_content)?;
    Ok(config)
}
