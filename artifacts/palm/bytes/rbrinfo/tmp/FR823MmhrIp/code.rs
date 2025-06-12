fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
        assert!(len <= self.remaining(), "`len` greater than remaining");

        let r = self.inner.copy_to_bytes(len);
        self.limit -= len;
        r
    }