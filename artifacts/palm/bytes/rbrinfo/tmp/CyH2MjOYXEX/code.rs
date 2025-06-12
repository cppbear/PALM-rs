fn remaining_mut(&self) -> usize {
        usize::MAX - self.len()
    }