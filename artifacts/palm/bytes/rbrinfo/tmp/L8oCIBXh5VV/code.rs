fn put_bytes(&mut self, val: u8, mut cnt: usize) {
        if self.remaining_mut() < cnt {
            panic_advance(&TryGetError {
                requested: cnt,
                available: self.remaining_mut(),
            })
        }

        while cnt > 0 {
            let dst = self.chunk_mut();
            let dst_len = usize::min(dst.len(), cnt);
            // SAFETY: The pointer is valid for `dst_len <= dst.len()` bytes.
            unsafe { core::ptr::write_bytes(dst.as_mut_ptr(), val, dst_len) };
            // SAFETY: We just initialized `dst_len` bytes in `self`.
            unsafe { self.advance_mut(dst_len) };
            cnt -= dst_len;
        }
    }