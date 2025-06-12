fn next(&mut self) -> Option<Self::Item> {
        self.drain.next().map(Bucket::key_value)
    }