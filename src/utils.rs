use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub fn validate_paths_and_updates_file_keys_match(
    updates: &HashMap<String, String>,
    paths: &[PathBuf],
) -> Result<(), String> {
    let mut path_keys = HashSet::new();
    let mut missing_keys = Vec::new();

    for path in paths {
        if let Some(file_str) = path.to_str() {
            let stem = get_file_stem(file_str)?;
            if !updates.contains_key(&stem) {
                missing_keys.push(stem.clone());
            }
            path_keys.insert(stem);
        }
    }

    let update_keys: HashSet<_> = updates.keys().cloned().collect();
    let mut extra_keys: Vec<_> = update_keys.difference(&path_keys).cloned().collect();

    if !missing_keys.is_empty() {
        if missing_keys.len() == 1 {
            return Err(format!("Updates file misses '{}' key!", missing_keys[0]));
        } else {
            missing_keys.sort();
            let keys = missing_keys.join("', '");
            return Err(format!("Updates file misses '{}' keys!", keys));
        }
    }

    if !extra_keys.is_empty() {
        if extra_keys.len() == 1 {
            return Err(format!(
                "Updates file has additional '{}' key!",
                extra_keys[0]
            ));
        } else {
            extra_keys.sort();
            let keys = extra_keys.join("', '");
            return Err(format!("Updates file has additional '{}' key!", keys));
        }
    }

    Ok(())
}

pub fn get_file_stem<P: AsRef<Path>>(path_str: P) -> Result<String, String> {
    let path = path_str.as_ref();

    if path.as_os_str().is_empty() {
        return Err("Can't get file stem! Empty string!".to_string());
    }

    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "Can't get file stem!".to_string())
}

pub fn get_path_and_key(s: &str) -> Result<(Vec<&str>, &str), String> {
    match s.rfind('.') {
        Some(pos) => {
            let (left, right) = s.split_at(pos);
            let path_parts: Vec<&str> = left.split('.').collect();
            Ok((path_parts, &right[1..]))
        }
        None => Ok((vec![], s)),
    }
}

#[cfg(test)]
mod validate_paths_and_updates_file_keys_match {
    use super::*;

    #[test]
    fn should_return_ok_when_paths_match_updates_file() {
        let mut updates: HashMap<String, String> = HashMap::new();
        updates.insert("en".to_string(), "en_value".to_owned());
        updates.insert("sv".to_string(), "sv_value".to_owned());
        let paths: Vec<PathBuf> = vec![
            PathBuf::from("files/en.json"),
            PathBuf::from("files/sv.json"),
        ];
        let result = validate_paths_and_updates_file_keys_match(&updates, &paths);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_return_err_if_updates_file_misses_one_key() {
        let mut updates: HashMap<String, String> = HashMap::new();
        updates.insert("en".to_string(), "en_value".to_owned());
        let paths: Vec<PathBuf> = vec![
            PathBuf::from("files/en.json"),
            PathBuf::from("files/sv.json"),
        ];
        let result = validate_paths_and_updates_file_keys_match(&updates, &paths);
        assert_eq!(result.unwrap_err(), "Updates file misses 'sv' key!");
    }

    #[test]
    fn should_return_err_if_updates_file_misses_multiple_keys() {
        let updates: HashMap<String, String> = HashMap::new();
        let paths: Vec<PathBuf> = vec![
            PathBuf::from("files/en.json"),
            PathBuf::from("files/sv.json"),
        ];
        let result = validate_paths_and_updates_file_keys_match(&updates, &paths);
        assert_eq!(result.unwrap_err(), "Updates file misses 'en', 'sv' keys!");
    }

    #[test]
    fn should_return_err_if_updates_file_has_additional_key() {
        let mut updates: HashMap<String, String> = HashMap::new();
        updates.insert("en".to_string(), "en_value".to_owned());
        updates.insert("sv".to_string(), "sv_value".to_owned());
        let paths: Vec<PathBuf> = vec![PathBuf::from("files/en.json")];
        let result = validate_paths_and_updates_file_keys_match(&updates, &paths);
        assert_eq!(result.unwrap_err(), "Updates file has additional 'sv' key!");
    }

    #[test]
    fn should_return_err_if_updates_file_has_multiple_additional_keys() {
        let mut updates: HashMap<String, String> = HashMap::new();
        updates.insert("en".to_string(), "en_value".to_owned());
        updates.insert("sv".to_string(), "sv_value".to_owned());
        let paths: Vec<PathBuf> = vec![];
        let result = validate_paths_and_updates_file_keys_match(&updates, &paths);
        assert_eq!(
            result.unwrap_err(),
            "Updates file has additional 'en', 'sv' key!"
        );
    }
}

#[cfg(test)]
mod get_file_stem {
    use super::*;

    #[test]
    fn should_return_file_stem_for_valid_paths() {
        let cases = [
            ("foo.json", "foo"),
            ("files/foo.json", "foo"),
            ("./files/foo.json", "foo"),
            ("foo", "foo"),
            ("files/foo", "foo"),
            ("./files/foo", "foo"),
        ];

        for (input, expected) in cases {
            let result = get_file_stem(input);
            assert_eq!(result.unwrap(), expected, "Failed on input: {}", input);
        }
    }

    #[test]
    fn should_return_error_for_empty_string() {
        let result = get_file_stem("");
        assert_eq!(result.unwrap_err(), "Can't get file stem! Empty string!");
    }
}
