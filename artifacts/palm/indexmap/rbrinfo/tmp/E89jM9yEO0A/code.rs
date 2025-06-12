pub fn last(&self) -> Option<(&K, &V)> {
        self.entries.last().map(Bucket::refs)
    }