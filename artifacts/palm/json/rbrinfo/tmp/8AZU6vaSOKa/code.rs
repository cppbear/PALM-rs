pub fn is_i64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_i64(),
            _ => false,
        }
    }