pub mod config;
pub mod runner;

use config::read_config;
use runner::run_commands;

pub fn execute_push_or_not(ignore_errors: bool) -> Result<(), String> {
    let commands =
        read_config("pre-push-config.toml").map_err(|e| format!("Failed to read config: {}", e))?;

    run_commands(commands, ignore_errors)
}
