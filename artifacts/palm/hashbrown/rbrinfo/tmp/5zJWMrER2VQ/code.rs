pub fn drain(&mut self) -> Drain<'_, T, A> {
        Drain {
            inner: self.raw.drain(),
        }
    }