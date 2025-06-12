pub fn last(&self) -> Option<(&K, &V)> {
        self.as_entries().last().map(Bucket::refs)
    }