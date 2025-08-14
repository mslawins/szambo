use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

use serde_json::Value;

pub fn load_json_into_hash_map<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let map: HashMap<String, String> = serde_json::from_str(&content)?;
    Ok(map)
}

pub fn load_json_into_value<P: AsRef<Path>>(path: &P) -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&content)?;
    Ok(json)
}

pub fn list_files_in_dir<P: AsRef<Path>>(
    path: &P,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}

pub fn save_value_to_json_file<P: AsRef<Path>>(json: &Value, path: &P) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    let json_string = serde_json::to_string_pretty(json)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}
