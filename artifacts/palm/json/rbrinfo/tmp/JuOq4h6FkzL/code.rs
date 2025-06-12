fn eq_u64(value: &Value, other: u64) -> bool {
    value.as_u64() == Some(other)
}