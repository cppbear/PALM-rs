fn put<T: super::Buf>(&mut self, mut src: T)
    where
        Self: Sized,
    {
        if self.remaining_mut() < src.remaining() {
            panic_advance(&TryGetError {
                requested: src.remaining(),
                available: self.remaining_mut(),
            });
        }

        while src.has_remaining() {
            let s = src.chunk();
            let d = self.chunk_mut();
            let cnt = usize::min(s.len(), d.len());

            d[..cnt].copy_from_slice(&s[..cnt]);

            // SAFETY: We just initialized `cnt` bytes in `self`.
            unsafe { self.advance_mut(cnt) };
            src.advance(cnt);
        }
    }