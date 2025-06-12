fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize {
        if self.limit == 0 {
            return 0;
        }

        const LEN: usize = 16;
        let mut slices: [IoSlice<'a>; LEN] = [
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
            IoSlice::new(&[]),
        ];

        let cnt = self
            .inner
            .chunks_vectored(&mut slices[..dst.len().min(LEN)]);
        let mut limit = self.limit;
        for (i, (dst, slice)) in dst[..cnt].iter_mut().zip(slices.iter()).enumerate() {
            if let Some(buf) = slice.get(..limit) {
                // SAFETY: We could do this safely with `IoSlice::advance` if we had a larger MSRV.
                let buf = unsafe { std::mem::transmute::<&[u8], &'a [u8]>(buf) };
                *dst = IoSlice::new(buf);
                return i + 1;
            } else {
                // SAFETY: We could do this safely with `IoSlice::advance` if we had a larger MSRV.
                let buf = unsafe { std::mem::transmute::<&[u8], &'a [u8]>(slice) };
                *dst = IoSlice::new(buf);
                limit -= slice.len();
            }
        }
        cnt
    }