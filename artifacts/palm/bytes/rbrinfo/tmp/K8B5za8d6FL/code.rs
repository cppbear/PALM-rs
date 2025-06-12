fn chunk(&self) -> &[u8] {
        let slice = self.get_ref().as_ref();
        let pos = min_u64_usize(self.position(), slice.len());
        &slice[pos..]
    }