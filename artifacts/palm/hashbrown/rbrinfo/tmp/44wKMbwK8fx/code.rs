pub unsafe fn as_ref<'a>(&self) -> &'a T {
        &*self.as_ptr()
    }