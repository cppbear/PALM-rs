fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }