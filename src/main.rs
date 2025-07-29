mod files;
mod json;
mod parser;
mod string;

use std::path::Path;

use clap::Parser;
use parser::{Cli, Commands};

use crate::json::{
    insert::insert_under_key, remove::remove_key_at_path, rename::rename_key_at_path,
    replace::replace_value_at_key,
};

fn get_file_stem<P: AsRef<Path>>(path_str: P) -> Option<String> {
    path_str
        .as_ref()
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|s| s.to_string())
}

fn get_path_and_key(s: &str) -> Result<(Vec<&str>, &str), String> {
    match s.rfind('.') {
        Some(pos) => {
            let (left, right) = s.split_at(pos);
            let path_parts: Vec<&str> = left.split('.').collect();
            Ok((path_parts, &right[1..]))
        }
        None => Ok((vec![], s)),
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
            let (path, new_key) = get_path_and_key(&key).unwrap();

            files.iter().for_each(|file| {
                let mut json = files::load_json_into_value(&file).unwrap();
                let hash_map_key = get_file_stem(&file).unwrap();
                let value = updates.get(&hash_map_key).unwrap();
                insert_under_key(&mut json, &path, &new_key, value).unwrap();
                files::save_value_to_json_file(&json, file).unwrap();
            });
        }

        Commands::Remove { key, where_ } => {
            println!("Removing key '{}' from '{}'", key, where_);
            let files = files::list_files_in_dir(where_).unwrap();
            let (path, key_to_remove) = get_path_and_key(&key).unwrap();

            files.iter().for_each(|file| {
                let mut json = files::load_json_into_value(&file).unwrap();
                remove_key_at_path(&mut json, &path, key_to_remove).unwrap();
                files::save_value_to_json_file(&json, file).unwrap();
            });
        }

        Commands::Replace { key, from, where_ } => {
            println!(
                "Replacing key '{}' with data from '{}' in '{}'",
                key, from, where_
            );
            let updates = files::load_json_into_hash_map(&from).unwrap();
            let files = files::list_files_in_dir(where_).unwrap();
            let (path, key_to_replace) = get_path_and_key(&key).unwrap();

            files.iter().for_each(|file| {
                let hash_map_key = get_file_stem(&file).unwrap();
                let new_value = updates.get(&hash_map_key).unwrap();

                let mut json = files::load_json_into_value(&file).unwrap();
                replace_value_at_key(&mut json, &path, key_to_replace, new_value).unwrap();
                files::save_value_to_json_file(&json, file).unwrap();
            });
        }

        Commands::Rename { from, to, where_ } => {
            println!("Renaming '{}' to '{}' in {}", from, to, where_);

            let from_path: Vec<&str> = from.split('.').collect();
            let to_path: Vec<&str> = to.split('.').collect();
            let files = files::list_files_in_dir(where_).unwrap();

            files.iter().for_each(|file| {
                let mut json = files::load_json_into_value(&file).unwrap();
                rename_key_at_path(&mut json, &from_path, &to_path).unwrap();
                files::save_value_to_json_file(&json, file).unwrap();
            });
        }

        Commands::Sort { where_ } => {
            println!("Sorting files in {}", where_);
            let files = files::list_files_in_dir(where_).unwrap();

            files.iter().for_each(|file| {
                let json = files::load_json_into_value(&file).unwrap();
                files::save_value_to_json_file(&json, file).unwrap();
            });
        }
    }
}
