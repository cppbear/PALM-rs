fn chunk_mut(&mut self) -> &mut UninitSlice {
        if self.capacity() == self.len() {
            self.reserve(64); // Grow the vec
        }

        let cap = self.capacity();
        let len = self.len();

        let ptr = self.as_mut_ptr();
        // SAFETY: Since `ptr` is valid for `cap` bytes, `ptr.add(len)` must be
        // valid for `cap - len` bytes. The subtraction will not underflow since
        // `len <= cap`.
        unsafe { UninitSlice::from_raw_parts_mut(ptr.add(len), cap - len) }
    }