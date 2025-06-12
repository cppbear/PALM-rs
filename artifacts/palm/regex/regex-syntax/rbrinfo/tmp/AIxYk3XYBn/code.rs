pub fn union(&mut self, other: &ClassUnicode) {
        self.set.union(&other.set);
    }