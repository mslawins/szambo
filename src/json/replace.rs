use serde_json::Value;

pub fn replace_value_at_key(
    json: &mut Value,
    path: &[&str],
    key: &str,
    new_value: &str,
) -> Result<(), String> {
    let mut current = json;

    for segment in path {
        current = current.get_mut(*segment).ok_or_else(|| {
            format!(
                "Expected path segment '{}' to exist, but it was not found.",
                segment
            )
        })?;

        if !current.is_object() {
            return Err(format!(
                "Expected object at path segment '{}', but found non-object.",
                segment
            ));
        }
    }

    match current.as_object_mut() {
        Some(obj) => {
            if obj.contains_key(key) {
                obj.insert(key.to_string(), Value::String(new_value.to_string()));
                Ok(())
            } else {
                Err(format!(
                    "Key '{}' does not exist at the target path! Use ADD command instead.",
                    key
                ))
            }
        }
        None => Err(
            "Expected an object at final path to replace value in, but found non-object."
                .to_string(),
        ),
    }
}

#[cfg(test)]
mod replace_value_at_key {
    use super::*;
    use serde_json::json;

    #[test]
    fn should_return_err_if_key_to_replace_does_not_exist() {
        let mut data = json!({});
        let expected = json!({});

        let result = replace_value_at_key(&mut data, &[], "new", "value");

        assert_eq!(data, expected);
        assert_eq!(
            result.unwrap_err(),
            "Key 'new' does not exist at the target path! Use ADD command instead.",
        );
    }

    #[test]
    fn should_replace_string_at_path_if_key_exists() {
        let mut data = json!({ "key": "old_value" });
        let expected = json!({ "key": "new_value" });

        let result = replace_value_at_key(&mut data, &[], "key", "new_value");

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn should_replace_object_at_path_if_key_exists() {
        let mut data = json!({
            "key": { "foo": "bar" }
        });
        let expected = json!({ "key": "new_value" });

        let result = replace_value_at_key(&mut data, &[], "key", "new_value");

        assert_eq!(data, expected);
        assert_eq!(result.unwrap(), ());
    }
}
