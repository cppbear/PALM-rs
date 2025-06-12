pub fn first(&self) -> Option<&T> {
        self.entries.first().map(Bucket::key_ref)
    }