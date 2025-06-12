fn as_non_null(&self) -> NonNull<T> {
        // SAFETY: `self.ptr` is already a `NonNull`
        unsafe { NonNull::new_unchecked(self.as_ptr()) }
    }