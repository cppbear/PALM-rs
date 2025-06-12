fn chunk(&self) -> &[u8] {
        let (s1, s2) = self.as_slices();
        if s1.is_empty() {
            s2
        } else {
            s1
        }
    }