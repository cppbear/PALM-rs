pub fn first(&self) -> Option<&T> {
        self.as_entries().first().map(Bucket::key_ref)
    }