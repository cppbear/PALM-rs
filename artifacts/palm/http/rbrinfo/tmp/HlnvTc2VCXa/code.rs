pub fn is<T: error::Error + 'static>(&self) -> bool {
        self.get_ref().is::<T>()
    }