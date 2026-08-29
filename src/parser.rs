use anyhow::{Context, Result, bail};
use serde_json::Value;

/// Accepted keys when the top-level JSON document is an object.
const OBJECT_KEYS: [&str; 3] = ["scores", "data", "values"];

/// Extracts a flat `Vec<f64>` of scores from a JSON document.
///
/// The input may be:
/// - a top-level JSON **array** of numbers, e.g. `[9.2, 8.4, 7.9]`, or
/// - a top-level JSON **object** holding one of the keys
///   `"scores"`, `"data"` or `"values"` as an array of numbers.
///
/// Returns an error for anything else (invalid JSON, non-numeric
/// elements, or an unsupported shape).
pub fn parse_scores(input: &str) -> Result<Vec<f64>> {
    let value: Value =
        serde_json::from_str(input).with_context(|| "input is not valid JSON".to_string())?;

    let items = match value {
        Value::Array(items) => items,
        Value::Object(map) => {
            let key = OBJECT_KEYS
                .iter()
                .find(|key| map.contains_key(**key))
                .with_context(|| {
                    format!(
                        "object must use one of these keys: {}",
                        OBJECT_KEYS.join(", ")
                    )
                })?;
            map.get(*key)
                .and_then(Value::as_array)
                .with_context(|| format!("\"{key}\" must be an array of numbers"))?
                .clone()
        }
        _ => bail!("expected a JSON array of numbers or an object"),
    };

    items
        .iter()
        .map(Value::as_f64)
        .collect::<Option<Vec<_>>>()
        .with_context(|| "all scores must be JSON numbers".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_an_array_of_numbers() {
        let scores = parse_scores("[9.2, 8.4, 7.9]").expect("valid input");
        assert_eq!(scores, vec![9.2, 8.4, 7.9]);
    }

    #[test]
    fn parses_an_object_with_scores_key() {
        let scores = parse_scores(r#"{"scores": [1, 2, 3]}"#).expect("valid input");
        assert_eq!(scores, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn rejects_non_numeric_elements() {
        let err = parse_scores(r#"[1, "two", 3]"#).unwrap_err();
        assert!(err.to_string().contains("all scores must be JSON numbers"));
    }

    #[test]
    fn rejects_scalar_top_level_json() {
        assert!(parse_scores("42").is_err());
        assert!(parse_scores("\"hello\"").is_err());
    }

    #[test]
    fn rejects_invalid_json() {
        assert!(parse_scores("{ not json").is_err());
    }
}
