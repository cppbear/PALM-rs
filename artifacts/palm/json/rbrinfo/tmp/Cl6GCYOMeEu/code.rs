fn eq_i64(value: &Value, other: i64) -> bool {
    value.as_i64() == Some(other)
}