fn partial_cmp(&self, other: &[u8]) -> Option<cmp::Ordering> {
        (*self.inner).partial_cmp(other)
    }