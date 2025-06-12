pub fn move_index(self, to: usize) {
        let index = self.index();
        self.into_ref_mut().move_index(index, to);
    }