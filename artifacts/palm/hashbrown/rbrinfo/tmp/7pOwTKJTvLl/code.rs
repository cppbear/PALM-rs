pub(crate) unsafe fn read(&self) -> T {
        self.as_ptr().read()
    }