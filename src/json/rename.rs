use serde_json::Value;

pub fn rename_key_at_path(
    json: &mut Value,
    from_path: &[&str],
    to_path: &[&str],
) -> Result<(), String> {
    if let Some(value) = take_value_at_path(json, from_path) {
        insert_value_at_path(json, to_path, value);
        Ok(())
    } else {
        Err(format!(
            "Key '{}' not found during rename!",
            from_path.join(".")
        ))
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

#[cfg(test)]
mod rename_key_at_path {
    use super::*;
    use serde_json::json;

    #[test]
    fn should_rename_key_at_path() {
        let mut data = json!({
            "key": "value"
        });
        let expected = json!({
            "new_key": "value"
        });

        let result = rename_key_at_path(&mut data, &["key"], &["new_key"]);

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_rename_nested_key_at_path() {
        let mut data = json!({
            "foo": {
                "key": "value"
            }
        });
        let expected = json!({
            "foo": {
                "new_key": "value"
            }
        });

        let result = rename_key_at_path(&mut data, &["foo", "key"], &["foo", "new_key"]);

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_rename_deeply_nested_key_at_path() {
        let mut data = json!({
            "foo": {
                "bar": {
                    "key": "value"
                }
            }
        });
        let expected = json!({
            "foo": {
                "bar": {
                    "new_key": "value"
                }
            }
        });

        let result = rename_key_at_path(
            &mut data,
            &["foo", "bar", "key"],
            &["foo", "bar", "new_key"],
        );

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_move_existing_key_to_parent_key() {
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
                "bar": "value"
            }
        });

        let result = rename_key_at_path(&mut data, &["foo", "bar", "key"], &["foo", "bar"]);

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_move_existing_key_as_parent_sibling() {
        let mut data = json!({
            "foo": {
                "bar": {
                    "key": "value"
                }
            }
        });
        let expected = json!({
            "foo": {
                "bar": {},
                "baz": "value"
            }
        });

        let result = rename_key_at_path(&mut data, &["foo", "bar", "key"], &["foo", "baz"]);

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_return_err_if_from_path_does_not_exist_in_file() {
        let mut data = json!({
            "foo": {
                "bar": {
                    "key": "value"
                }
            }
        });
        let expected = json!({
            "foo": {
                "bar": {
                    "key": "value"
                }
            }
        });

        let result = rename_key_at_path(&mut data, &["foo", "bar", "other_key"], &["foo", "baz"]);

        assert_eq!(data, expected);
        assert_eq!(
            result.unwrap_err(),
            "Key 'foo.bar.other_key' not found during rename!"
        );
    }
}
