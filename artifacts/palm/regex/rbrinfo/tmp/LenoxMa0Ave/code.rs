fn reset_size(&mut self) {
        self.size =
            (self.start_states.len() * mem::size_of::<StatePtr>())
            + (self.stack.len() * mem::size_of::<InstPtr>());
    }