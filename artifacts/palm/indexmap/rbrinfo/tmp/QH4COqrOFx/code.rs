pub fn last(&self) -> Option<&T> {
        self.entries.last().map(Bucket::key_ref)
    }