mod files;
mod json;
mod parser;
mod utils;

use clap::Parser;
use parser::{Cli, Commands};

use crate::json::{
    compare::get_missing_paths, insert::insert_under_key, remove::remove_key_at_path,
    rename::rename_key_at_path, replace::replace_value_at_key,
};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::AddToMany {
            key,
            from,
            where_,
            files,
        } => {
            if let Some(required_keys) = files {
                let required_keys = utils::parse_limit(&required_keys).unwrap();
                println!(
                    "Adding from: '{}' to directory: '{}' under key: {} only for {:?}",
                    from, where_, key, required_keys
                );
                let updates = files::load_json_into_hash_map(&from).unwrap();
                let files = files::list_files_in_dir(where_).unwrap();
                utils::validate_required_keys_exist(&updates, &files, &required_keys).unwrap();
                let (path, new_key) = utils::get_path_and_key(&key).unwrap();

                files
                    .iter()
                    .filter(|file| {
                        let stem = utils::get_file_stem(&file).unwrap();
                        required_keys.contains(&stem)
                    })
                    .for_each(|file| {
                        let mut json = files::load_json_into_value(&file).unwrap();
                        let hash_map_key = utils::get_file_stem(&file).unwrap();
                        let value = updates.get(&hash_map_key).unwrap();
                        insert_under_key(&mut json, &path, &new_key, value).unwrap();
                        files::save_value_to_json_file(&json, file).unwrap();
                    });
            } else {
                println!(
                    "Adding from: '{}' to: '{}' under key: {}",
                    from, where_, key
                );
                let updates = files::load_json_into_hash_map(&from).unwrap();
                let files = files::list_files_in_dir(where_).unwrap();
                utils::validate_paths_and_updates_file_keys_match(&updates, &files).unwrap();
                let (path, new_key) = utils::get_path_and_key(&key).unwrap();

                files.iter().for_each(|file| {
                    let mut json = files::load_json_into_value(&file).unwrap();
                    let hash_map_key = utils::get_file_stem(&file).unwrap();
                    let value = updates.get(&hash_map_key).unwrap();
                    insert_under_key(&mut json, &path, &new_key, value).unwrap();
                    files::save_value_to_json_file(&json, file).unwrap();
                });
            }
        }

        Commands::AddToSingle { from, where_ } => {
            println!("Adding from {} into {}", from, where_);
            let updates = files::load_json_into_hash_map(&from).unwrap();
            let mut json = files::load_json_into_value(&where_).unwrap();

            updates.iter().for_each(|(full_path, value)| {
                let (path, new_key) = utils::get_path_and_key(&full_path).unwrap();
                insert_under_key(&mut json, &path, &new_key, value).unwrap();
            });
            files::save_value_to_json_file(&json, &where_).unwrap();
        }

        Commands::Remove { key, where_ } => {
            println!("Removing key '{}' from '{}'", key, where_);
            let files = files::list_files_in_dir(where_).unwrap();
            let (path, key_to_remove) = utils::get_path_and_key(&key).unwrap();

            files.iter().for_each(|file| {
                let mut json = files::load_json_into_value(&file).unwrap();
                let result = remove_key_at_path(&mut json, &path, key_to_remove);
                if let Err(message) = result {
                    println!("{} for file: {:?}", message, file);
                }
                files::save_value_to_json_file(&json, file).unwrap();
            });
        }

        Commands::Replace {
            key,
            from,
            where_,
            files,
        } => {
            if let Some(required_keys) = files {
                let required_keys = utils::parse_limit(&required_keys).unwrap();
                println!(
                    "Replacing key '{}' with data from '{}' in '{} only for {:?}'",
                    key, from, where_, required_keys
                );
                let updates = files::load_json_into_hash_map(&from).unwrap();
                let files = files::list_files_in_dir(where_).unwrap();
                utils::validate_required_keys_exist(&updates, &files, &required_keys).unwrap();
                let (path, key_to_replace) = utils::get_path_and_key(&key).unwrap();

                files
                    .iter()
                    .filter(|file| {
                        let stem = utils::get_file_stem(&file).unwrap();
                        required_keys.contains(&stem)
                    })
                    .for_each(|file| {
                        let hash_map_key = utils::get_file_stem(&file).unwrap();
                        let new_value = updates.get(&hash_map_key).unwrap();

                        let mut json = files::load_json_into_value(&file).unwrap();
                        replace_value_at_key(&mut json, &path, key_to_replace, new_value).unwrap();
                        files::save_value_to_json_file(&json, file).unwrap();
                    });
            } else {
                println!(
                    "Replacing key '{}' with data from '{}' in '{}'",
                    key, from, where_
                );
                let updates = files::load_json_into_hash_map(&from).unwrap();
                let files = files::list_files_in_dir(where_).unwrap();
                utils::validate_paths_and_updates_file_keys_match(&updates, &files).unwrap();
                let (path, key_to_replace) = utils::get_path_and_key(&key).unwrap();

                files.iter().for_each(|file| {
                    let hash_map_key = utils::get_file_stem(&file).unwrap();
                    let new_value = updates.get(&hash_map_key).unwrap();

                    let mut json = files::load_json_into_value(&file).unwrap();
                    replace_value_at_key(&mut json, &path, key_to_replace, new_value).unwrap();
                    files::save_value_to_json_file(&json, file).unwrap();
                });
            }
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

        Commands::Compare { reference, target } => {
            println!(
                "Comparing target file: {} to reference file: {}",
                target, reference
            );
            let reference = files::load_json_into_value(reference).unwrap();
            let target = files::load_json_into_value(target).unwrap();
            let result = get_missing_paths(&reference, &target);
            println!("{}", result);
        }

        Commands::CompareAll { where_ } => {
            println!("Comparing all files in directory: {}", where_);
            let files = files::list_files_in_dir(&where_).unwrap();

            if files.len() < 2 {
                println!(
                    "Not enough files to compare in directory: {}. Directory contains {} files",
                    where_,
                    files.len()
                );
            }

            let reference = files.get(0).unwrap();
            let reference = files::load_json_into_value(reference).unwrap();

            let mut failed = false;
            files.iter().skip(1).for_each(|file| {
                let target = files::load_json_into_value(file).unwrap();
                let result = get_missing_paths(&reference, &target);
                println!("{}", result);

                if result.is_there_any_difference() {
                    failed = true;
                }
            });

            if failed == true {
                std::process::exit(1);
            }
        }
    }
}
