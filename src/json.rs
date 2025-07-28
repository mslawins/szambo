use serde_json::Value;

// TODO: Test Err paths
pub fn insert_under_key(
    json: &mut Value,
    path: &[&str],
    new_key: &str,
    new_value: &str,
) -> Result<(), String> {
    let mut current = json;

    for key in path {
        if !current.is_object() {
            return Err(format!(
                "Expected object at path segment '{}', found something else",
                key
            ));
        }

        if current.get(*key).is_none() {
            current
                .as_object_mut()
                .ok_or("Expected object while traversing path".to_string())?
                .insert((*key).to_string(), Value::Object(Default::default()));
        }

        current = current
            .as_object_mut()
            .and_then(|obj| obj.get_mut(*key))
            .ok_or_else(|| format!("Path segment '{}' not found", key))?;
    }

    if let Some(obj) = current.as_object_mut() {
        if obj.contains_key(new_key) {
            return Err(format!(
                "Key '{}' already exists at the target path! Use REPLACE command instead.",
                new_key
            ));
        }

        obj.insert(new_key.to_string(), Value::String(new_value.to_string()));
        Ok(())
    } else {
        Err("Expected an object at final path to insert into, but found non-object.".to_string())
    }
}

#[cfg(test)]
mod insert_under_key {
    use super::*;
    use serde_json::json;

    #[test]
    fn should_insert_value_in_empty_json_file() {
        let mut data = json!({});
        let expected = json!({
            "new": "value"
        });

        let result = insert_under_key(&mut data, &[], "new", "value");

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_insert_before_existing_key_if_alphabetically_earlier() {
        let mut data = json!({ "a": "a_value" });
        let expected = json!({
            "a": "a_value",
            "new": "value"
        });

        let result = insert_under_key(&mut data, &[], "new", "value");

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_insert_after_existing_key_if_alphabetically_later() {
        let mut data = json!({ "m": "m_value" });
        let expected = json!({
            "new": "value",
            "m": "m_value"
        });

        let result = insert_under_key(&mut data, &[], "new", "value");

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_insert_value_at_path_if_path_exists() {
        let mut data = json!({
            "foo": {}
        });
        let expected = json!({
            "foo": {
                "new": "value"
            }
        });

        let result = insert_under_key(&mut data, &["foo"], "new", "value");

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_insert_value_at_path_if_path_does_not_exists() {
        let mut data = json!({});
        let expected = json!({
            "foo": {
                "new": "value"
            }
        });

        let result = insert_under_key(&mut data, &["foo"], "new", "value");

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_insert_value_at_deep_path_if_path_does_not_exists() {
        let mut data = json!({});
        let expected = json!({
            "foo": {
                "bar": {
                    "new": "value"
                }
            }
        });

        let result = insert_under_key(&mut data, &["foo", "bar"], "new", "value");

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_return_error_if_there_is_object_under_key() {
        let mut data = json!({
            "new": {
                "inner": "inner_value"
            }
        });
        let expected = json!({
            "new": {
                "inner": "inner_value"
            }
        });

        let result = insert_under_key(&mut data, &[], "new", "value");

        assert_eq!(data, expected);
        assert_eq!(
            result.unwrap_err(),
            "Key 'new' already exists at the target path! Use REPLACE command instead.",
        );
    }

    #[test]
    fn should_return_error_if_there_is_string_under_key() {
        let mut data = json!({
            "new": "new_value"

        });
        let expected = json!({
            "new": "new_value"
        });

        let result = insert_under_key(&mut data, &[], "new", "value");

        assert_eq!(data, expected);
        assert_eq!(
            result.unwrap_err(),
            "Key 'new' already exists at the target path! Use REPLACE command instead.",
        );
    }
}
