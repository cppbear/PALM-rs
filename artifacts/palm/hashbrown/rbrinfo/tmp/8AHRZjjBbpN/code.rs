pub fn as_ptr(&self) -> *mut T {
        if T::IS_ZERO_SIZED {
            // Just return an arbitrary ZST pointer which is properly aligned
            // invalid pointer is good enough for ZST
            invalid_mut(mem::align_of::<T>())
        } else {
            unsafe { self.ptr.as_ptr().sub(1) }
        }
    }