fn chunks_vectored<'a>(&'a self, dst: &mut [io::IoSlice<'a>]) -> usize {
        if self.is_empty() || dst.is_empty() {
            return 0;
        }

        let (s1, s2) = self.as_slices();
        dst[0] = io::IoSlice::new(s1);
        if s2.is_empty() || dst.len() == 1 {
            return 1;
        }

        dst[1] = io::IoSlice::new(s2);
        2
    }