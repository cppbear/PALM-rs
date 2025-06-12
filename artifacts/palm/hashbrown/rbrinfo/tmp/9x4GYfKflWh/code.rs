pub fn get_mut(&mut self) -> &mut T {
        unsafe { self.bucket.as_mut() }
    }