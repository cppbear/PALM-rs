fn chunk_mut(&mut self) -> &mut UninitSlice {
        UninitSlice::uninit(self)
    }