use serde_json::Value;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct JsonDiff {
    pub missing_in_target: Vec<String>,
    pub missing_in_reference: Vec<String>,
}

impl fmt::Display for JsonDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\nMissing in target:\n")?;
        if self.missing_in_target.is_empty() {
            writeln!(f, "-")?;
        } else {
            for path in &self.missing_in_target {
                writeln!(f, "{}", path)?;
            }
        }

        writeln!(f, "\nMissing in reference:\n")?;
        if self.missing_in_reference.is_empty() {
            writeln!(f, "-")?;
        } else {
            for path in &self.missing_in_reference {
                writeln!(f, "{}", path)?;
            }
        }

        Ok(())
    }
}

pub fn get_missing_paths(reference_json: &Value, target_json: &Value) -> JsonDiff {
    let mut reference_paths = BTreeSet::new();
    let mut target_paths = BTreeSet::new();

    collect_paths(reference_json, String::new(), &mut reference_paths);
    collect_paths(target_json, String::new(), &mut target_paths);

    let missing_in_target = reference_paths
        .difference(&target_paths)
        .cloned()
        .collect::<Vec<_>>();

    let missing_in_reference = target_paths
        .difference(&reference_paths)
        .cloned()
        .collect::<Vec<_>>();

    JsonDiff {
        missing_in_target,
        missing_in_reference,
    }
}

fn collect_paths(value: &Value, current_path: String, paths: &mut BTreeSet<String>) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let new_path = if current_path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", current_path, key)
                };
                collect_paths(val, new_path, paths);
            }
        }
        _ => {
            paths.insert(current_path);
        }
    }
}

#[cfg(test)]
mod get_missing_paths {
    use super::*;
    use serde_json::json;

    #[test]
    fn should_return_empty_json_diff_for_two_empty_files() {
        let reference = json!({});
        let target = json!({});
        let expected = JsonDiff {
            missing_in_reference: vec![],
            missing_in_target: vec![],
        };

        let result = get_missing_paths(&reference, &target);

        assert_eq!(result, expected);
    }

    #[test]
    fn should_return_empty_json_diff_for_identical_files() {
        let reference = json!({
            "foo": {
                "bar": "value",
                "baz": "value",
            }
        });
        let target = json!({
            "foo": {
                "bar": "other_value",
                "baz": "yet_another_value",
            }
        });
        let expected = JsonDiff {
            missing_in_reference: vec![],
            missing_in_target: vec![],
        };

        let result = get_missing_paths(&reference, &target);

        assert_eq!(result, expected);
    }

    #[test]
    fn should_return_json_diff_where_key_is_missing_in_target() {
        let reference = json!({ "key": "value" });
        let target = json!({});
        let expected = JsonDiff {
            missing_in_reference: vec![],
            missing_in_target: vec!["key".to_owned()],
        };

        let result = get_missing_paths(&reference, &target);

        assert_eq!(result, expected);
    }

    #[test]
    fn should_return_json_diff_where_key_is_missing_in_reference() {
        let reference = json!({});
        let target = json!({ "key": "value" });
        let expected = JsonDiff {
            missing_in_reference: vec!["key".to_owned()],
            missing_in_target: vec![],
        };

        let result = get_missing_paths(&reference, &target);

        assert_eq!(result, expected);
    }

    #[test]
    fn should_return_json_diff_where_nested_key_is_missing_in_target() {
        let reference = json!({
            "foo": {
                "bar": "value",
                "baz": "value"
            }
        });
        let target = json!({
            "foo": {
                "bar": "value",
            }
        });
        let expected = JsonDiff {
            missing_in_reference: vec![],
            missing_in_target: vec!["foo.baz".to_owned()],
        };

        let result = get_missing_paths(&reference, &target);

        assert_eq!(result, expected);
    }

    #[test]
    fn should_return_json_diff_where_nested_key_is_missing_in_reference() {
        let reference = json!({
            "foo": {
                "bar": "value",
            }
        });
        let target = json!({
            "foo": {
                "bar": "value",
                "baz": "value"
            }
        });
        let expected = JsonDiff {
            missing_in_reference: vec!["foo.baz".to_owned()],
            missing_in_target: vec![],
        };

        let result = get_missing_paths(&reference, &target);

        assert_eq!(result, expected);
    }

    #[test]
    fn should_return_json_diff_where_reference_and_target_files_have_missing_keys() {
        let reference = json!({
            "foo": {
                "bar": "value",
            },
            "key": "value"
        });
        let target = json!({
            "foo": {
                "bar": "value",
                "baz": "value"
            }
        });
        let expected = JsonDiff {
            missing_in_reference: vec!["foo.baz".to_owned()],
            missing_in_target: vec!["key".to_owned()],
        };

        let result = get_missing_paths(&reference, &target);

        assert_eq!(result, expected);
    }
}
