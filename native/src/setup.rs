use std::env::current_exe;
use std::io;
use std::io::{BufRead, Error};
use serde_json::json;
use crate::registry::create_registry;

pub fn setup() -> Result<(), Error> {
    let manifest_path = create_manifest()?;
    create_registry(manifest_path)?;
    Ok(())
}

pub fn create_manifest() -> Result<String, Error> {
    let binding = current_exe()?;
    let current_exe_str = binding.to_str().unwrap();
    let stdin = io::stdin();
    println!("enter your chrome extension id:");
    let id = stdin.lock().lines().next().unwrap()?;
    let manifest = json!({
        "name": "de.tamion.web_runnables",
        "description": "Run local commands from your Browser",
        "path": current_exe_str,
        "type": "stdio",
        "allowed_origins": [format!("chrome-extension://{id}/")]
    });
    let manifest_path = current_exe()?.with_file_name("manifest.json");
    let manifest_file = std::fs::File::create(&manifest_path)?;
    serde_json::to_writer_pretty(manifest_file, &manifest)?;
    Ok(manifest_path.to_str().unwrap().to_string())
}

