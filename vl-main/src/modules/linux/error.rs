use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinuxModuleError {
    #[error(
        "Failed to connect into the Linux Backend Socket. Reason: {0}"
    )]
    FailedToConnectIntoSocket(String),
    #[error("Linux Backend Service was not started")]
    BackendServiceNotStarted,
    #[error("Failed to get devices: {0}")]
    FailedToGetDevices(String),
    #[error("Failed to link: {0}")]
    FailedToLink(String),
    #[error("Failed to unlink: {0}")]
    FailedToUnlink(String),
    #[error("Failed to initialize speaking: {0}")]
    FailedToSpeak(String),
}
