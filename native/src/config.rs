use std::env::current_exe;
use std::io::Error;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub fn load_config() -> Result<Config, Error> {
    let path = current_exe()?.with_file_name("config.json");
    let config_file = std::fs::File::open(path);
    match config_file {
        Ok(file) => {
            let mut config = serde_json::from_reader(file)?;
            merge(&mut config, default_config());
            let config = serde_json::from_value::<Config>(config)?;
            save_config(&config);
            Ok(config)
        }
        Err(_) => {
            let config = serde_json::from_value::<Config>(default_config())?;
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

fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}

fn default_config() -> Value {
    json!({
        "require_special": true,
        "runnables": [
            {
                "name": "hello",
                "regex": "*",
                "hotkey": "Ctrl+Shift+E",
                "command": "echo Hello World"
            }
        ]
    })
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub require_special: bool,
    pub runnables: Vec<Runnable>,
}

#[derive(Serialize, Deserialize)]
pub struct Runnable {
    pub name: String,
    pub regex: String,
    pub hotkey: String,
    pub command: String,
}
