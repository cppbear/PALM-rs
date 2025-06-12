fn next(&mut self) -> Option<T> {
        unsafe { Some(self.iter.next()?.read()) }
    }