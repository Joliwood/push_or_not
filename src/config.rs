use std::fs;
use toml::Value;

pub fn read_config(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let parsed: Value = toml::from_str(&content)?;

    let commands = parsed["commands"]["pre_push"]
        .as_array()
        .ok_or("Invalid config format")?
        .iter()
        .map(|cmd| cmd.as_str().unwrap().to_string())
        .collect();

    Ok(commands)
}
