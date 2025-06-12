fn chunks_vectored<'b>(&'b self, dst: &mut [IoSlice<'b>]) -> usize {
            (**self).chunks_vectored(dst)
        }