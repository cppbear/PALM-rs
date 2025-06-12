pub fn first(&self) -> Option<(&K, &V)> {
        self.entries.first().map(Bucket::refs)
    }