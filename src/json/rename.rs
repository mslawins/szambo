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
            "Key '{}' not found during rename.",
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
