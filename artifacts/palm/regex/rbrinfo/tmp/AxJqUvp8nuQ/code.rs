fn num_states(&self) -> usize {
        self.table.len() / self.num_byte_classes
    }