fn chunk_mut(&mut self) -> &mut UninitSlice {
        if self.a.has_remaining_mut() {
            self.a.chunk_mut()
        } else {
            self.b.chunk_mut()
        }
    }