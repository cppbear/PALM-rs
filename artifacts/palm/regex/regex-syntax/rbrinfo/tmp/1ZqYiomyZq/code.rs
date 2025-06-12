pub fn difference(&mut self, other: &ClassUnicode) {
        self.set.difference(&other.set);
    }