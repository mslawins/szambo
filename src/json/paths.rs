use serde_json::Value;

pub fn get_json_paths(value: &Value) -> Result<Vec<String>, String> {
    let mut paths = Vec::new();
    collect_paths(value, String::new(), &mut paths)?;
    Ok(paths)
}

fn collect_paths(value: &Value, prefix: String, paths: &mut Vec<String>) -> Result<(), String> {
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                let new_prefix = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                collect_paths(v, new_prefix, paths)?;
            }
            Ok(())
        }
        Value::String(_) => {
            paths.push(prefix);
            Ok(())
        }
        other => Err(format!(
            "Unexpected value type at '{}': {:?}",
            prefix, other
        )),
    }
}
