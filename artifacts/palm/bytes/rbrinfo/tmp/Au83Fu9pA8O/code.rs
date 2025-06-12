fn try_copy_to_slice(&mut self, mut dst: &mut [u8]) -> Result<(), TryGetError> {
        if self.remaining() < dst.len() {
            return Err(TryGetError {
                requested: dst.len(),
                available: self.remaining(),
            });
        }

        while !dst.is_empty() {
            let src = self.chunk();
            let cnt = usize::min(src.len(), dst.len());

            dst[..cnt].copy_from_slice(&src[..cnt]);
            dst = &mut dst[cnt..];

            self.advance(cnt);
        }
        Ok(())
    }