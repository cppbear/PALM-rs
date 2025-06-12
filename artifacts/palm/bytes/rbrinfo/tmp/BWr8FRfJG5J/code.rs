fn drop(&mut self) {
        unsafe { dealloc(self.buf, Layout::from_size_align(self.cap, 1).unwrap()) }
    }