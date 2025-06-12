pub fn union(&mut self, other: &ClassBytes) {
        self.set.union(&other.set);
    }