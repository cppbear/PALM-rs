pub fn union(&mut self, other: &IntervalSet<I>) {
        // This could almost certainly be done more efficiently.
        self.ranges.extend(&other.ranges);
        self.canonicalize();
    }