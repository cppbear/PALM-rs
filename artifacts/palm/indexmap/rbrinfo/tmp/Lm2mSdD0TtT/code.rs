pub(crate) fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.borrow_mut().shift_remove_index(index)
    }