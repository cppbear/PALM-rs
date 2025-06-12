pub fn values(&self) -> Values<'_, K, V> {
        Values::new(&self.entries)
    }