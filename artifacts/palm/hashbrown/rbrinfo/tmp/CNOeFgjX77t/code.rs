fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.map.extend(iter.into_iter().map(|k| (k, ())));
    }