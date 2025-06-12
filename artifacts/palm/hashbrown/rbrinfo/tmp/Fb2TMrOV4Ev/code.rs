unsafe fn next_n(&self, offset: usize) -> Self {
        let ptr = if T::IS_ZERO_SIZED {
            // invalid pointer is good enough for ZST
            invalid_mut(self.ptr.as_ptr() as usize + offset)
        } else {
            self.ptr.as_ptr().sub(offset)
        };
        Self {
            ptr: NonNull::new_unchecked(ptr),
        }
    }