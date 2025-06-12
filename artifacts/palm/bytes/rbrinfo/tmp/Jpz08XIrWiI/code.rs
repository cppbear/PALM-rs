fn remaining(&self) -> usize {
        cmp::min(self.inner.remaining(), self.limit)
    }