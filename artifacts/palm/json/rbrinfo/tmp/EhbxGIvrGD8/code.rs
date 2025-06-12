fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (String, Value)>,
    {
        self.map.extend(iter);
    }