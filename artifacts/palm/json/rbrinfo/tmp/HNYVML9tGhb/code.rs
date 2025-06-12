pub fn is_u64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_u64(),
            _ => false,
        }
    }