fn eq_str(value: &Value, other: &str) -> bool {
    value.as_str() == Some(other)
}