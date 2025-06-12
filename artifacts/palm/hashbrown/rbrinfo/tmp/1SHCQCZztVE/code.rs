pub unsafe fn as_mut<'a>(&self) -> &'a mut T {
        &mut *self.as_ptr()
    }