fn chunk(&self) -> &[u8] {
        if self.a.has_remaining() {
            self.a.chunk()
        } else {
            self.b.chunk()
        }
    }