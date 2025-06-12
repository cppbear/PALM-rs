fn state(&self, si: StatePtr) -> &State {
        &self.cache.states[si as usize / self.num_byte_classes()]
    }