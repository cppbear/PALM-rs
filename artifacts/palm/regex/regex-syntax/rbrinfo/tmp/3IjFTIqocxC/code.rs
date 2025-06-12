pub fn difference(&mut self, other: &ClassBytes) {
        self.set.difference(&other.set);
    }