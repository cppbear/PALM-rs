pub fn get(&self) -> Option<&'a T> {
        let ptr = self.inner.load(Ordering::Acquire);
        unsafe { ptr.as_ref() }
    }