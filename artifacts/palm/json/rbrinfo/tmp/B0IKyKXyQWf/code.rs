fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        self[..].index_or_insert(v)
    }