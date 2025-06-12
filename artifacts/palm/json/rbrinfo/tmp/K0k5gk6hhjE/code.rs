fn from(f: f32) -> Self {
        Number::from_f32(f).map_or(Value::Null, Value::Number)
    }