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
                "Path segment '{}' does not exist in the JSON structure.",
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
