pub(crate) unsafe fn write(&self, val: T) {
        self.as_ptr().write(val);
    }