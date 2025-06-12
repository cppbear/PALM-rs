fn index(&self, idx: usize) -> &Self::Output {
        unsafe { &(*self.0)[idx].links }
    }