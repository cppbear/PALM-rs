fn remaining_mut(&self) -> usize {
        cmp::min(self.inner.remaining_mut(), self.limit)
    }