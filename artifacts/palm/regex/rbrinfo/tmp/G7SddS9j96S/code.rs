fn state_heap_size(&self) -> usize {
        self.num_byte_classes * mem::size_of::<StatePtr>()
    }