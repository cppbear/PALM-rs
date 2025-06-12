unsafe fn advance_mut(&mut self, cnt: usize) {
        assert!(cnt <= self.limit);
        self.inner.advance_mut(cnt);
        self.limit -= cnt;
    }