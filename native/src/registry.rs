use std::env::current_exe;
use std::io::{stderr, stdout, Error};
use std::process::Command;
use serde_json::json;

pub fn create_registry(manifest_path: String) -> Result<(), Error> {
    Command::new("pwsh")
        .arg("-CommandWithArgs")
        .arg(format!("REG ADD \"HKCU\\Software\\Google\\Chrome\\NativeMessagingHosts\\de.tamion.web_runnables\" /ve /t REG_SZ /d \"{}\" /f", manifest_path))
        .stdout(stdout())
        .stderr(stderr())
        .spawn()?;
    Ok(())
}
