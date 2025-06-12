pub fn last(&self) -> Option<&T> {
        self.as_entries().last().map(Bucket::key_ref)
    }