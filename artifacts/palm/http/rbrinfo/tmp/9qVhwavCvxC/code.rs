fn eq(&self, other: &HeaderMap<T>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.keys()
            .all(|key| self.get_all(key) == other.get_all(key))
    }