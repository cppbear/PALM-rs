pub fn first(&self) -> Option<(&K, &V)> {
        self.as_entries().first().map(Bucket::refs)
    }