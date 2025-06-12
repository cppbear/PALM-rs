pub fn intersect(&mut self, other: &ClassBytes) {
        self.set.intersect(&other.set);
    }