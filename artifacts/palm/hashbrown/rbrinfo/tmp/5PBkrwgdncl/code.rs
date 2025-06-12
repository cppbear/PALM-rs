fn next(&mut self) -> Option<T> {
        unsafe {
            let item = self.iter.next()?;
            Some(item.read())
        }
    }