pub fn get_or_insert<T: Clone + Send + Sync + 'static>(&mut self, value: T) -> &mut T {
        self.get_or_insert_with(|| value)
    }