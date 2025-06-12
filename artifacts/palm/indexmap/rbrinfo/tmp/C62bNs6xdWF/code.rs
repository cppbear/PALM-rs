pub(crate) fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.borrow_mut().swap_remove_index(index)
    }