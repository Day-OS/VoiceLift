use std::collections::HashMap;
use std::process::{Command, ExitStatus};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinkError {
    #[error("Failed to execute command: {0}")]
    CommandExecutionError(#[from] std::io::Error),
    #[error("Command failed with status: {0}")]
    CommandFailedError(ExitStatus),
}

pub fn link(input: &String, output: &String) -> Result<(), LinkError> {
    let mut command = Command::new("pw-link");
    command.arg(output).arg(input);
    match command.status() {
        Ok(status) => {
            if status.success() {
                return Ok(());
            } else {
                return Err(LinkError::CommandFailedError(status));
            }
        }
        Err(e) => return Err(LinkError::CommandExecutionError(e)),
    }
}

#[derive(Error, Debug)]
pub enum UnlinkError {
    #[error("Failed to execute command: {0}")]
    CommandExecutionError(#[from] std::io::Error),
    #[error("Command failed with status: {0}")]
    CommandFailedError(ExitStatus),
}

pub fn unlink(input: &String, output: &String) -> Result<(), UnlinkError> {
    let mut command = Command::new("pw-link");
    command.arg("--disconnect").arg(output).arg(input);
    match command.status() {
        Ok(status) => {
            if status.success() {
                return Ok(());
            } else {
                return Err(UnlinkError::CommandFailedError(status));
            }
        }
        Err(e) => return Err(UnlinkError::CommandExecutionError(e)),
    }
}

#[derive(Error, Debug)]
pub enum ScanDeviceError {
    #[error("Failed to execute command: {0}")]
    CommandExecutionError(#[from] std::io::Error),
    #[error("Command failed with status: {0}")]
    CommandFailedError(ExitStatus),
    #[error("Could not scan channels: {0}")]
    ScanChannelError(#[from] ScanChannelError),
    #[error("Device ignored! Reason: {0}")]
    DeviceIgnored(String),
    #[error("Unexpected error occurred: {0}")]
    UnexpectedError(String),
}

pub fn get_pw_entries() -> Result<Vec<HashMap<String, String>>, ScanDeviceError> {
    let mut command = Command::new("pw-cli");
    command.arg("ls").arg("Node");
    let output = command.output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);

    let mut entries = Vec::new();
    let mut current_entry = HashMap::new();

    for line in output_str.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("id ") {
            if !current_entry.is_empty() {
                entries.push(current_entry);
                current_entry = HashMap::new();
            }
        } else {
            let mut parts = line.splitn(2, " = ");

            let key = match parts.next() {
                Some(k) => k.trim().to_string(),
                None => continue,
            };

            let value = match parts.next() {
                Some(k) => k
                    .trim()
                    .strip_prefix("\"")
                    .unwrap()
                    .strip_suffix("\"")
                    .unwrap()
                    .to_string(),
                None => continue,
            };

            current_entry.insert(key, value);
        }
    }

    if !current_entry.is_empty() {
        entries.push(current_entry);
    }

    Ok(entries)
}

#[derive(Error, Debug)]
pub enum ScanChannelError {
    #[error("Failed to parse output")]
    ParseError,
    #[error("Failed to execute command")]
    CommandError(#[from] std::io::Error),
    #[error("No channels found for device {0}")]
    NoChannelsFound(String),
}

pub fn _scan_channels(arg: &str) -> Result<Vec<String>, ScanChannelError> {
    let mut command = Command::new("pw-link");
    let output = command.arg(arg).output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut channels = Vec::new();
    for line in output_str.lines() {
        let line = line.trim();
        channels.push(line.to_owned());
    }
    Ok(channels)
}
pub fn scan_input_channels() -> Result<Vec<String>, ScanChannelError> {
    _scan_channels("-i")
}
pub fn scan_output_channels() -> Result<Vec<String>, ScanChannelError> {
    _scan_channels("-o")
}
