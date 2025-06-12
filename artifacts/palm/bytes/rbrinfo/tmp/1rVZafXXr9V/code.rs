fn drop(&mut self) {
        unsafe { (self.vtable.drop)(&mut self.data, self.ptr, self.len) }
    }