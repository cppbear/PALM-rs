fn put_slice(&mut self, mut src: &[u8]) {
        if self.remaining_mut() < src.len() {
            panic_advance(&TryGetError {
                requested: src.len(),
                available: self.remaining_mut(),
            });
        }

        while !src.is_empty() {
            let dst = self.chunk_mut();
            let cnt = usize::min(src.len(), dst.len());

            dst[..cnt].copy_from_slice(&src[..cnt]);
            src = &src[cnt..];

            // SAFETY: We just initialized `cnt` bytes in `self`.
            unsafe { self.advance_mut(cnt) };
        }
    }