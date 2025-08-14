use crate::files;
use crate::json::compare::get_missing_paths;
use crate::json::insert::insert_under_key;
use crate::json::paths::get_json_paths;
use crate::json::remove::remove_key_at_path;
use crate::json::rename::rename_key_at_path;
use crate::json::replace::replace_value_at_key;
use crate::search::find_unused_paths;
use crate::utils;

pub fn add_to_many_command(key: String, from: String, where_: String, files: Option<String>) {
    let updates = files::load_json_into_hash_map(&from).unwrap();
    let files_in_dir = files::list_files_in_dir(&where_).unwrap();
    let (path, new_key) = utils::get_path_and_key(&key).unwrap();

    if let Some(required_keys) = files {
        let required_keys = utils::parse_limit(&required_keys).unwrap();

        println!(
            "Adding from: '{}' to directory: '{}' under key: {} only for {:?}",
            from, where_, key, required_keys
        );

        utils::validate_required_keys_exist(&updates, &files_in_dir, &required_keys).unwrap();

        files_in_dir
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
        utils::validate_paths_and_updates_file_keys_match(&updates, &files_in_dir).unwrap();

        files.iter().for_each(|file| {
            let mut json = files::load_json_into_value(&file).unwrap();
            let hash_map_key = utils::get_file_stem(&file).unwrap();
            let value = updates.get(&hash_map_key).unwrap();
            insert_under_key(&mut json, &path, &new_key, value).unwrap();
            files::save_value_to_json_file(&json, file).unwrap();
        });
    }
}

pub fn add_to_single_command(from: String, where_: String) {
    println!("Adding from {} into {}", from, where_);

    let updates = files::load_json_into_hash_map(&from).unwrap();
    let mut json = files::load_json_into_value(&where_).unwrap();

    updates.iter().for_each(|(full_path, value)| {
        let (path, new_key) = utils::get_path_and_key(&full_path).unwrap();
        insert_under_key(&mut json, &path, &new_key, value).unwrap();
    });
    files::save_value_to_json_file(&json, &where_).unwrap();
}

pub fn remove_command(key: String, where_: String) {
    println!("Removing key '{}' from '{}'", key, where_);
    let files = files::list_files_in_dir(&where_).unwrap();
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

pub fn replace_command(key: String, from: String, where_: String, files: Option<String>) {
    let updates = files::load_json_into_hash_map(&from).unwrap();
    let files_in_dir = files::list_files_in_dir(&where_).unwrap();
    let (path, key_to_replace) = utils::get_path_and_key(&key).unwrap();

    if let Some(required_keys) = files {
        let required_keys = utils::parse_limit(&required_keys).unwrap();
        println!(
            "Replacing key '{}' with data from '{}' in '{} only for {:?}'",
            key, from, where_, required_keys
        );
        utils::validate_required_keys_exist(&updates, &files_in_dir, &required_keys).unwrap();

        files_in_dir
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
        utils::validate_paths_and_updates_file_keys_match(&updates, &files_in_dir).unwrap();

        files_in_dir.iter().for_each(|file| {
            let hash_map_key = utils::get_file_stem(&file).unwrap();
            let new_value = updates.get(&hash_map_key).unwrap();

            let mut json = files::load_json_into_value(&file).unwrap();
            replace_value_at_key(&mut json, &path, key_to_replace, new_value).unwrap();
            files::save_value_to_json_file(&json, file).unwrap();
        });
    }
}

pub fn rename_command(from: String, to: String, where_: String) {
    println!("Renaming '{}' to '{}' in {}", from, to, where_);

    let from_path: Vec<&str> = from.split('.').collect();
    let to_path: Vec<&str> = to.split('.').collect();
    let files = files::list_files_in_dir(&where_).unwrap();

    files.iter().for_each(|file| {
        let mut json = files::load_json_into_value(&file).unwrap();
        rename_key_at_path(&mut json, &from_path, &to_path).unwrap();
        files::save_value_to_json_file(&json, file).unwrap();
    });
}

pub fn sort_command(where_: String) {
    println!("Sorting files in {}", where_);
    let files = files::list_files_in_dir(&where_).unwrap();

    files.iter().for_each(|file| {
        let json = files::load_json_into_value(&file).unwrap();
        files::save_value_to_json_file(&json, file).unwrap();
    });
}

pub fn compare_command(target: String, reference: String) {
    println!(
        "Comparing target file: {} to reference file: {}",
        target, reference
    );
    let reference_json = files::load_json_into_value(&reference).unwrap();
    let target_json = files::load_json_into_value(&target).unwrap();
    let result = get_missing_paths(&reference_json, &target_json, &reference, &target);
    println!("{}", result);
}

pub fn compare_all_command(where_: String) {
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
    let reference_json = files::load_json_into_value(reference).unwrap();

    let mut failed = false;

    files.iter().skip(1).for_each(|file| {
        let target_json = files::load_json_into_value(file).unwrap();
        let reference_str = reference.to_str().unwrap();
        let target_str = file.to_str().unwrap();
        let result = get_missing_paths(&reference_json, &target_json, &reference_str, &target_str);
        println!("{}", result);

        if result.is_there_any_difference() {
            failed = true;
        }
    });

    if failed == true {
        std::process::exit(1);
    }
}

pub fn list_unused_keys_command(translations: String, source: String) {
    println!(
        "Searching for unused keys in directory: {} based on translations file: {}",
        source, translations
    );

    let json = files::load_json_into_value(&translations).unwrap();
    let paths = get_json_paths(&json).unwrap();
    let unused_paths = find_unused_paths(paths, source).unwrap();

    println!("Unused paths (some might be false positives!):\n",);

    for unused_path in &unused_paths {
        println!("{}", unused_path)
    }
}

pub fn remove_unused_keys_command(translations: String, source: String, where_: String) {
    println!(
        "Searching for unused keys in directory: {} based on translations file: {}",
        source, translations
    );

    let json = files::load_json_into_value(&translations).unwrap();
    let paths = get_json_paths(&json).unwrap();
    let unused_paths = find_unused_paths(paths, source).unwrap();
    let files = files::list_files_in_dir(&where_).unwrap();

    println!("Removing unused paths!");

    files.iter().for_each(|file| {
        let mut json = files::load_json_into_value(&file).unwrap();
        for unused_path in &unused_paths {
            let (path, key_to_remove) = utils::get_path_and_key(unused_path).unwrap();

            let result = remove_key_at_path(&mut json, &path, key_to_remove);
            if let Err(message) = result {
                println!("{} for file: {:?}", message, file);
            }
        }
        files::save_value_to_json_file(&json, file).unwrap();
    });
}
