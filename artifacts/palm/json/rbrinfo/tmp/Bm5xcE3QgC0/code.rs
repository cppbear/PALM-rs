fn eq_bool(value: &Value, other: bool) -> bool {
    value.as_bool() == Some(other)
}