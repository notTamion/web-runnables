use std::error::Error;
use std::io;
use std::env::args;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use regex::Regex;
use serde_json::{json, Value};
use crate::config::load_config;
use crate::setup::setup;

mod config;
mod registry;
mod setup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
        match message["type"].as_str().unwrap() {
            "run" => {
                let id = message["id"].as_u64().unwrap() as usize;
                let runnable= config.runnables.get(id).unwrap().clone();
                let args = message["args"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect::<Vec<&str>>();
                let command = format_string(runnable.command.clone().as_str(), args);
                let process = Command::new("pwsh")
                    .arg("-CommandWithArgs")
                    .arg(command)
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();
                tokio::spawn(async move {
                    let mut output = String::new();
                    process.stdout.unwrap().read_to_string(&mut output).unwrap();
                    log(output).expect("TODO: panic message");
                });
            }
            _ => {}
        }
    }
}

fn format_string(template: &str, replacements: Vec<&str>) -> String {
    let re = Regex::new(r"\{(\d+)}").unwrap();

    re.replace_all(template, |caps: &regex::Captures| {
        let index: usize = caps[1].parse().unwrap();

        replacements.get(index).unwrap_or(&"").to_string()
    })
        .to_string()
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
