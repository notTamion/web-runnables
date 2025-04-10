use std::error::Error;
use std::io;
use std::env::args;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use serde_json::{json, Value};
use crate::config::{load_config, Runnable};
use crate::setup::setup;

mod config;
mod registry;
mod setup;

fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
    if args().find(|arg| arg.contains("chrome-extension")).is_none() {
        setup()?;
        return Ok(());
    }
    write_output(serde_json::to_string_pretty(&json!({
        "type": "config",
        "value": config,
    })).unwrap().as_str()).expect("FAILED TO SERIALIZE CONFIG");
    loop {
        let message = serde_json::from_str::<Value>(&*String::from_utf8_lossy(&*read_input()?).to_string())?;
        log(message.get("type").unwrap().as_str().unwrap().to_string()).expect("TODO: panic message");
        match message["type"].as_str().unwrap() {
            "run" => {
                let id = message["id"].as_u64().unwrap() as usize;
                let runnable= &config.runnables.get(id);
                let runnable = runnable.unwrap();
                let mut process = Command::new("pwsh")
                    .arg("-CommandWithArgs")
                    .arg(&runnable.command)
                    .output()?;
                log(String::from_utf8_lossy(&*process.stdout).to_string()).expect("TODO: panic message");
            }
            _ => {}
        }
    }
}

pub fn log(msg: String) -> io::Result<()> {
    Ok(write_output(serde_json::to_string_pretty(
        &json!({
            "type": "log",
            "value": msg,
        })
    )?.as_str()).expect("FAILED TO SERIALIZE LOG"))
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
