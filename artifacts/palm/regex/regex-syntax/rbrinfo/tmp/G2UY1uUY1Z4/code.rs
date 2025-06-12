pub fn push(&mut self, interval: I) {
        // TODO: This could be faster. e.g., Push the interval such that
        // it preserves canonicalization.
        self.ranges.push(interval);
        self.canonicalize();
    }