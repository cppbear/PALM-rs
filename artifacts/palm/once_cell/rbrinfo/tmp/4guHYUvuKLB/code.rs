pub fn get_mut(&mut self) -> Option<&mut T> {
            // Safe because we have unique access
            unsafe { &mut *self.inner.get() }.as_mut()
        }