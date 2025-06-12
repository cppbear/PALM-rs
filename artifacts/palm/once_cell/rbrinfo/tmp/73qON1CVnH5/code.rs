fn eq(&self, other: &OnceCell<T>) -> bool {
            self.get() == other.get()
        }