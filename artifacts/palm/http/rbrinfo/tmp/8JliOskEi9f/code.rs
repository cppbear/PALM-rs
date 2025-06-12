fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        unsafe { &mut (*self.0)[idx].links }
    }