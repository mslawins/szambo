mod files;
mod json;
mod parser;

use std::path::Path;

use clap::Parser;
use parser::{Cli, Commands};
use serde_json::Value;

use crate::json::{insert::insert_under_key, remove::remove_key_at_path};

fn get_file_stem<P: AsRef<Path>>(path_str: P) -> Option<String> {
    path_str
        .as_ref()
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|s| s.to_string())
}

fn split_at_last_dot(s: &str) -> Option<(Vec<&str>, &str)> {
    match s.rfind('.') {
        Some(pos) => {
            let (left, right) = s.split_at(pos);
            let path_parts: Vec<&str> = left.split('.').collect();
            Some((path_parts, &right[1..])) // skip the dot
        }
        None => None,
    }
}

fn take_value_at_path<'a>(json: &'a mut Value, path: &[&str]) -> Option<Value> {
    if path.is_empty() {
        return None;
    }

    let mut current = json;

    for key in &path[..path.len() - 1] {
        current = current.get_mut(*key)?;
    }

    current.as_object_mut()?.remove(path[path.len() - 1])
}

fn insert_value_at_path(json: &mut Value, path: &[&str], value: Value) {
    let mut current = json;

    for key in &path[..path.len() - 1] {
        current = current
            .as_object_mut()
            .unwrap()
            .entry(*key)
            .or_insert_with(|| Value::Object(Default::default()));
    }

    if let Some(obj) = current.as_object_mut() {
        obj.insert(path[path.len() - 1].to_string(), value);
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { key, from, where_ } => {
            println!(
                "Adding from: '{}' to: '{}' under key: {}",
                from, where_, key
            );
            let updates = files::load_json_into_hash_map(&from).unwrap();
            let files = files::list_files_in_dir(where_).unwrap();

            files.iter().for_each(|file| {
                let mut translations_for_lang = files::load_json_into_value(&file).unwrap();
                let (path, new_key) = split_at_last_dot(&key).unwrap();
                let hash_map_key = get_file_stem(&file).unwrap();
                let value = updates.get(&hash_map_key).unwrap();
                let _ =
                    insert_under_key(&mut translations_for_lang, &path, &new_key, value).unwrap();
                let _ = files::save_value_to_json_file(&translations_for_lang, file);
            });
        }
        Commands::Remove { key, where_ } => {
            println!("Removing key '{}' from '{}'", key, where_);
            let files = files::list_files_in_dir(where_).unwrap();

            files.iter().for_each(|file| {
                let mut json = files::load_json_into_value(&file).unwrap();

                if let Some((path, key_to_remove)) = split_at_last_dot(&key) {
                    if let Err(err) = remove_key_at_path(&mut json, &path, key_to_remove) {
                        eprintln!("Error removing key: {}", err);
                    }
                }

                let _ = files::save_value_to_json_file(&json, file);
            });
        }

        Commands::Replace { key, from, where_ } => {
            println!(
                "Replacing key '{}' with data from '{}' in '{}'",
                key, from, where_
            );
            let updates = files::load_json_into_hash_map(&from).unwrap();
            let files = files::list_files_in_dir(where_).unwrap();

            files.iter().for_each(|file| {
                let mut translations = files::load_json_into_value(&file).unwrap();
                if let Some((path, target_key)) = split_at_last_dot(&key) {
                    let hash_map_key = get_file_stem(&file).unwrap();
                    let new_value = updates.get(&hash_map_key).unwrap();

                    let mut current = &mut translations;
                    for segment in &path[..path.len()] {
                        current = current.get_mut(*segment).unwrap_or_else(|| {
                            panic!("Path {:?} does not exist in file {:?}", path, file)
                        });
                    }

                    if let Some(obj) = current.as_object_mut() {
                        obj.insert(target_key.to_string(), Value::String(new_value.to_string()));
                    }
                }

                let _ = files::save_value_to_json_file(&translations, file);
            });
        }

        Commands::Rename { from, to, where_ } => {
            println!("Renaming '{}' to '{}' in {:?}", from, to, where_);

            let from_path: Vec<&str> = from.split('.').collect();
            let to_path: Vec<&str> = to.split('.').collect();
            let files = files::list_files_in_dir(where_).unwrap();

            files.iter().for_each(|file| {
                let mut json = files::load_json_into_value(&file).unwrap();

                if let Some(value) = take_value_at_path(&mut json, &from_path) {
                    insert_value_at_path(&mut json, &to_path, value);
                    let _ = files::save_value_to_json_file(&json, file);
                } else {
                    println!("Warning: key '{}' not found in {:?}", from, file);
                }
            });
        }
        Commands::Sort { where_ } => {
            let files = files::list_files_in_dir(where_).unwrap();

            files.iter().for_each(|file| {
                let json = files::load_json_into_value(&file).unwrap();
                let _ = files::save_value_to_json_file(&json, file).unwrap();
            });
        }
    }
}
