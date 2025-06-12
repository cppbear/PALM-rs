fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, _) = self.entries.size_hint();
        // There could be more than just the entries upper, as there
        // could be items in the `extra_values`. We could guess, saying
        // `upper + extra_values.len()`, but that could overestimate by a lot.
        (lower, None)
    }