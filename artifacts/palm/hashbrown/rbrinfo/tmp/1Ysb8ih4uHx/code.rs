pub fn drain(&mut self) -> RawDrain<'_, T, A> {
        unsafe {
            let iter = self.iter();
            self.drain_iter_from(iter)
        }
    }