pub fn set(&self, value: bool) -> Result<(), ()> {
        self.inner.set(Self::to_usize(value))
    }