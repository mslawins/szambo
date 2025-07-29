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
            obj.insert(key.to_string(), Value::String(new_value.to_string()));
            Ok(())
        }
        None => Err(
            "Expected an object at final path to replace value in, but found non-object."
                .to_string(),
        ),
    }
}
