fn index_mut(&mut self, index: I) -> &mut Value {
        index.index_or_insert(self)
    }