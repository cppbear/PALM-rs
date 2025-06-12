fn from(f: &[T]) -> Self {
        Value::Array(f.iter().cloned().map(Into::into).collect())
    }