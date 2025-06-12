pub fn symmetric_difference(&mut self, other: &ClassBytes) {
        self.set.symmetric_difference(&other.set);
    }