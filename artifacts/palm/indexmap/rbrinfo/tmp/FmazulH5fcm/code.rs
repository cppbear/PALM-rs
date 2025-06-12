fn index(&self, index: usize) -> &T {
        self.get_index(index).unwrap_or_else(|| {
            panic!(
                "index out of bounds: the len is {len} but the index is {index}",
                len = self.len()
            );
        })
    }