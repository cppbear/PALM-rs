pub fn into_mut(self) -> &'a mut T {
        unsafe { self.bucket.as_mut() }
    }