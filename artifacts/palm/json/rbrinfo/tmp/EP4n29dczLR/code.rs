fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v {
            Value::Array(vec) => vec.get_mut(*self),
            _ => None,
        }
    }