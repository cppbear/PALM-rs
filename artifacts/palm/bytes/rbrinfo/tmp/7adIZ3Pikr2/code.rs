fn remaining_mut(&self) -> usize {
        // A vector can never have more than isize::MAX bytes
        core::isize::MAX as usize - self.len()
    }