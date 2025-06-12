pub fn pop(&mut self) -> Option<T> {
        self.map.pop().map(|(x, ())| x)
    }