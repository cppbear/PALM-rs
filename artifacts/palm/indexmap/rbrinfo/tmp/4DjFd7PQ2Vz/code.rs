fn index_mut(&mut self, index: usize) -> &mut V {
        let len: usize = self.len();

        self.get_index_mut(index)
            .unwrap_or_else(|| {
                panic!("index out of bounds: the len is {len} but the index is {index}");
            })
            .1
    }