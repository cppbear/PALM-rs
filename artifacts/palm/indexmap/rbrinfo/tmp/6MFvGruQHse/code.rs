pub fn replace(&mut self, value: T) -> Option<T> {
        self.replace_full(value).1
    }