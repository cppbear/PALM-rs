fn chunk(&self) -> &[u8] {
        let bytes = self.inner.chunk();
        &bytes[..cmp::min(bytes.len(), self.limit)]
    }