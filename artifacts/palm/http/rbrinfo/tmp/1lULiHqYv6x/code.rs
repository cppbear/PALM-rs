fn drop(&mut self) {
        for _ in self.by_ref() {}
    }