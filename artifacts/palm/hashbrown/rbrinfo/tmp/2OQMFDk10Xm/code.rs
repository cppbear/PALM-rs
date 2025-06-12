pub fn drain(&mut self) -> Drain<'_, K, V, A> {
        Drain {
            inner: self.table.drain(),
        }
    }