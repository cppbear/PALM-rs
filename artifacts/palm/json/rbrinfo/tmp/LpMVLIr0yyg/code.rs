fn eq_f64(value: &Value, other: f64) -> bool {
    value.as_f64() == Some(other)
}