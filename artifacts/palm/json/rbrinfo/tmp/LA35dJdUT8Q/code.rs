pub fn as_object(&self) -> Option<&Map<String, Value>> {
        match self {
            Value::Object(map) => Some(map),
            _ => None,
        }
    }