pub fn iter(&self) -> ValueIter<'a, T> {
        // This creates a new GetAll struct so that the lifetime
        // isn't bound to &self.
        GetAll {
            map: self.map,
            index: self.index,
        }
        .into_iter()
    }