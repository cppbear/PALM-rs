fn clone(&self) -> Bytes {
        unsafe { (self.vtable.clone)(&self.data, self.ptr, self.len) }
    }