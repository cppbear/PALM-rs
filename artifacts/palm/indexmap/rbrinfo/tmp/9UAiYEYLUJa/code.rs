pub fn swap_indices(self, other: usize) {
        let index = self.index();
        self.into_ref_mut().swap_indices(index, other);
    }