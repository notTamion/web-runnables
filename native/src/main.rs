use std::error::Error;
use std::{fs, io};
use std::borrow::Cow;
use std::env::{args, current_exe};
use std::io::{stderr, stdin, stdout, BufRead, Read, Write};
use std::process::Command;
use serde_json::json;
use crate::config::{load_config, save_config, Runnable};
use crate::registry::create_registry;
use crate::setup::setup;

mod config;
mod registry;
mod setup;

fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config().unwrap();
    if args().find(|arg| arg.contains("chrome-extension")).is_none() {
        setup()?;
        return Ok(());
    }
    loop {
        let msg = read_input()?;
        write_output(&String::from_utf8_lossy(&*msg))?;
    }

    loop {
        for line in stdin().lock().lines() {
            let line = line?;
            let args: Vec<&str> = line.split(" ").collect();
            match args[0] {
                "run" => {
                    let name = args[1];
                    let command = config.runnables.iter().find(|r| r.name == name);
                    match command {
                        Some(runnable) => {
                            println!("Yes");
                            let output = Command::new("cmd")
                                .arg("/c")
                                .arg(&runnable.command)
                                .stdout(stdout())
                                .spawn();
                        }
                        None => {
                            eprintln!("Runnable not found: {}", name);
                        }
                    }
                }
                "exit" => {
                    return Ok(())
                }
                _ => {}
            }
        }
    }
}

pub fn create_manifest() -> Result<String, std::io::Error> {
    let binding = current_exe()?;
    let current_exe_str = binding.to_str().unwrap();
    let manifest = json!({
        "name": "de.tamion.web_runnables",
        "description": "Run local commands from your Browser",
        "path": current_exe_str,
        "type": "stdio",
        "allowed_origins": ["chrome-extension://bnbdcflpeaebhnfmkpaelaihgodloiip/"]
    });
    let manifest_path = current_exe()?.with_file_name("manifest.json");
    if manifest_path.exists() {
        return Ok("".to_string())
    }
    let manifest_file = std::fs::File::create(&manifest_path)?;
    serde_json::to_writer_pretty(manifest_file, &manifest)?;
    Ok(manifest_path.to_str().unwrap().to_string())
}

pub fn write_output(msg: &str) -> io::Result<()> {
    let mut outstream = io::stdout();
    let len = msg.len();
    let len = if len > 1024 * 1024 {
        let msg = format!("Message was too large, length: {}", len);
        return Err(io::Error::other(msg));
    } else {
        len as u32
    };
    outstream.write(&len.to_ne_bytes())?;
    outstream.write_all(msg.as_bytes())?;
    outstream.flush()?;
    Ok(())
}
pub fn read_input() -> io::Result<Vec<u8>> {
    let mut instream = io::stdin();
    let mut length = [0; 4];
    instream.read(&mut length)?;
    let mut buffer = vec![0; u32::from_ne_bytes(length) as usize];
    instream.read_exact(&mut buffer)?;
    Ok(buffer)
}
