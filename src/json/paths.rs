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
#[cfg(test)]
mod get_json_paths {
    use super::*;
    use serde_json::json;

    #[test]
    fn should_return_no_paths_for_empty_json_file() {
        let mut data = json!({});
        let expected: Vec<String> = vec![];

        let result = get_json_paths(&mut data);

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn should_return_all_paths() {
        let mut data = json!({
            "foo": "value",
            "bar": {
                "baz": "value",
            },
            "aaa": {},
            "bbb": {
                "ccc": {
                    "ddd": "value"
                }
            }
        });
        let expected: Vec<String> = vec![
            "bar.baz".to_owned(),
            "bbb.ccc.ddd".to_owned(),
            "foo".to_owned(),
        ];

        let result = get_json_paths(&mut data);

        assert_eq!(result.unwrap(), expected);
    }
}
