fn index_mut(&mut self, index: usize) -> &mut V {
        &mut self.entries[index].value
    }