fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        let n = cmp::min(self.buf.remaining_mut(), src.len());

        self.buf.put_slice(&src[..n]);
        Ok(n)
    }