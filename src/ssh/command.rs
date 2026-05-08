use std::os::unix::process::CommandExt;
use std::process::Command;

use crate::config::model::{AuthType, Host};

/// Build the full argv list for an SSH connection, optionally including sshpass.
pub fn build_ssh_command(host: &Host, password: Option<&str>) -> Vec<String> {
    let mut args = Vec::new();

    if host.auth_type == AuthType::Password
        && let Some(pwd) = password
        && is_sshpass_available()
    {
        args.push("sshpass".to_string());
        args.push("-p".to_string());
        args.push(pwd.to_string());
    }

    args.push("ssh".to_string());
    args.push("-p".to_string());
    args.push(host.port.to_string());

    if let Some(ref key_path) = host.key_path {
        args.push("-i".to_string());
        args.push(key_path.clone());
    }

    if let Some(ref extra) = host.extra_ssh_args {
        for arg in extra {
            args.push("-o".to_string());
            args.push(arg.clone());
        }
    }

    args.push(format!("{}@{}", host.username, host.address));
    args
}

/// Replace the current process with the SSH command (does not return on success).
pub fn exec_ssh(host: &Host, password: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let args = build_ssh_command(host, password);
    if args.is_empty() {
        return Err("Empty command".into());
    }

    let err = Command::new(&args[0]).args(&args[1..]).exec();
    Err(err.into())
}

/// Check whether the `sshpass` utility is installed.
pub fn is_sshpass_available() -> bool {
    Command::new("which")
        .arg("sshpass")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
