pub fn get(&self) -> &T {
        unsafe { self.bucket.as_ref() }
    }