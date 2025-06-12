fn set_failed(&mut self, _failed: &mut bool) {
        self.slice = &self.slice[..self.index];
    }