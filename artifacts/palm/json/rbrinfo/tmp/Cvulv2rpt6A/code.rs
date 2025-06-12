fn eq_f32(value: &Value, other: f32) -> bool {
    match value {
        Value::Number(n) => n.as_f32() == Some(other),
        _ => false,
    }
}