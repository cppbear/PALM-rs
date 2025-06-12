fn from(array: [T; N]) -> Self {
        Value::Array(array.into_iter().map(Into::into).collect())
    }