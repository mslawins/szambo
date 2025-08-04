use serde_json::Value;

pub fn remove_key_at_path(
    json: &mut Value,
    path: &[&str],
    key_to_remove: &str,
) -> Result<(), String> {
    let mut current = json;

    for segment in path {
        current = current.get_mut(*segment).ok_or_else(|| {
            format!(
                "Path segment '{}' does not exist in the JSON structure!",
                segment
            )
        })?;
    }

    match current {
        Value::Object(obj) => {
            obj.remove(key_to_remove);
            Ok(())
        }
        _ => Err("Expected object at final path, but found non-object.".to_string()),
    }
}

#[cfg(test)]
mod remove_key_at_path {
    use super::*;
    use serde_json::json;

    #[test]
    fn should_remove_last_key_from_file() {
        let mut data = json!({ "key": "value" });
        let expected = json!({});

        let result = remove_key_at_path(&mut data, &[], "key");

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_remove_only_matching_key() {
        let mut data = json!({ "key": "value", "other_key": "value" });
        let expected = json!({ "other_key": "value" });

        let result = remove_key_at_path(&mut data, &[], "key");

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_remove_nested_matching_key() {
        let mut data = json!({
            "foo": {
                "key": "value"
            }
        });
        let expected = json!({
            "foo": {}
        });

        let result = remove_key_at_path(&mut data, &["foo"], "key");

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_remove_deeply_nested_matching_key() {
        let mut data = json!({
            "foo": {
                "bar": {
                    "key": "value"
                }
            }
        });
        let expected = json!({
            "foo": {
                "bar": {}
            }
        });

        let result = remove_key_at_path(&mut data, &["foo", "bar"], "key");

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_remove_whole_object_under_key() {
        let mut data = json!({
            "foo": {
                "bar": {
                    "key": "value",
                    "other_key": "value"
                }
            }
        });
        let expected = json!({
            "foo": {
            }
        });

        let result = remove_key_at_path(&mut data, &["foo"], "bar");

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_return_err_if_path_does_not_match_file_shape() {
        let mut data = json!({
            "foo": {
                "bar": {
                    "key": "value",
                }
            }
        });
        let expected = json!({
            "foo": {
                "bar": {
                    "key": "value",
                }
            }
        });

        let result = remove_key_at_path(&mut data, &["foo", "baz"], "key");

        assert_eq!(data, expected);
        assert_eq!(
            result.unwrap_err(),
            "Path segment 'baz' does not exist in the JSON structure!",
        );
    }

    #[test]
    fn should_return_do_nothing_if_path_and_key_do_not_match_file_shape() {
        let mut data = json!({
            "foo": {
                "bar": {
                    "key": "value",
                }
            }
        });
        let expected = json!({
            "foo": {
                "bar": {
                    "key": "value",
                }
            }
        });

        let result = remove_key_at_path(&mut data, &["foo", "bar"], "other_key");

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }
}
