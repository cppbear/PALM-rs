fn advance(&mut self, cnt: usize) {
        assert!(cnt <= self.limit);
        self.inner.advance(cnt);
        self.limit -= cnt;
    }