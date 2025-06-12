pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        index.index_into(self)
    }