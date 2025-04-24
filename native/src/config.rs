use std::env::current_exe;
use std::error::Error;
use serde::{Deserialize, Serialize};

pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let path = current_exe()?.with_file_name("config.yml");
    let config_file = std::fs::File::open(path);
    match config_file {
        Ok(file) => {
            let config = serde_yaml::from_reader(file)?;
            Ok(config)
        }
        Err(_) => {
            let config = default_config();
            save_config(&config);
            Ok(config)
        }
    }
}

pub fn save_config(config: &Config) {
    let path = current_exe().unwrap().with_file_name("config.yml");
    let file = std::fs::File::create(path).unwrap();
    serde_yaml::to_writer(file, &config).unwrap();
}

fn default_config() -> Config {
    serde_yaml::from_str::<Config>(r#"
    require_special: false
    runnables:
      - name: "Example"
        regex: ".*"
        hotkey: "Ctrl+Shift+E"
        command: "echo Hello, World!"
    "#).unwrap()
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub require_special: bool,
    pub runnables: Vec<Runnable>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Runnable {
    pub name: String,
    pub regex: String,
    #[serde(default)]
    pub arg_parser: String,
    pub hotkey: String,
    pub command: String,
}
