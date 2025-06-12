fn remaining_mut(&self) -> usize {
        self.a
            .remaining_mut()
            .saturating_add(self.b.remaining_mut())
    }