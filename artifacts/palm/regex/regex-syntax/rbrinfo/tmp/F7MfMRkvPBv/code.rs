pub fn symmetric_difference(&mut self, other: &IntervalSet<I>) {
        // TODO(burntsushi): Fix this so that it amortizes allocation.
        let mut intersection = self.clone();
        intersection.intersect(other);
        self.union(other);
        self.difference(&intersection);
    }