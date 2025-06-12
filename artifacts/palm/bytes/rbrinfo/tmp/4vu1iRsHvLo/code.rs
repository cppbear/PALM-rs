fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize {
        let mut n = self.a.chunks_vectored(dst);
        n += self.b.chunks_vectored(&mut dst[n..]);
        n
    }