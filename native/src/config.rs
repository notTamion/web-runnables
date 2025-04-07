use std::env::current_exe;
use std::io::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn load_config() -> Result<Config, Error> {
    let path = current_exe()?.with_file_name("config.json");
    let config_file = std::fs::File::open(path);
    match config_file {
        Ok(file) => {
            let config = serde_json::from_reader(file)?;
            Ok(config)
        }
        Err(_) => {
            let config = default_config()?;
            save_config(&config);
            Ok(config)
        }
    }
}

pub fn save_config(config: &Config) {
    let path = current_exe().unwrap().with_file_name("config.json");
    let file = std::fs::File::create(path).unwrap();
    serde_json::to_writer_pretty(file, &config).unwrap();
}

fn default_config() -> Result<Config, Error> {
    Ok(serde_json::from_value(json!({
        "runnables": [
            {
                "name": "hello",
                "regex": "*",
                "hotkey": "Ctrl+Shift+E",
                "command": "echo Hello World"
            }
        ]
    }))?)
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub runnables: Vec<Runnable>,
}

#[derive(Serialize, Deserialize)]
pub struct Runnable {
    pub name: String,
    pub regex: String,
    pub hotkey: String,
    pub command: String,
}
