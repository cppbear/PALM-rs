pub fn drain(&mut self) -> Drain<'_, T, A> {
        Drain {
            iter: self.map.drain(),
        }
    }