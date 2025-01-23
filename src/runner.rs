use std::process::Command;

pub fn run_commands(commands: Vec<String>, ignore_errors: bool) -> Result<(), String> {
    for command in commands {
        println!("Running: {}", command);
        let status = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .status()
            .map_err(|e| format!("Failed to execute {}: {}", command, e))?;

        if !status.success() && !ignore_errors {
            return Err(format!("Command `{}` failed", command));
        }
    }
    Ok(())
}
