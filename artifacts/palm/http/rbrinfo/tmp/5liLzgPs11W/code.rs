fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)> {
            Sealed::find(&self.as_str(), map)
        }