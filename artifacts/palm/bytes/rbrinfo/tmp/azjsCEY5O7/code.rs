pub fn is_unique(&self) -> bool {
        unsafe { (self.vtable.is_unique)(&self.data) }
    }