pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        unsafe {
            let ptr = self.ptr.as_ptr().add(self.len);
            let len = self.cap - self.len;

            slice::from_raw_parts_mut(ptr.cast(), len)
        }
    }