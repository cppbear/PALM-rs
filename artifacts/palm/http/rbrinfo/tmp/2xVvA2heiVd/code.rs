fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)> {
            map.find(self)
        }