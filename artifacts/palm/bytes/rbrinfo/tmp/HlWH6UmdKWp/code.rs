fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = u8>,
    {
        let iter = iter.into_iter();

        let (lower, _) = iter.size_hint();
        self.reserve(lower);

        // TODO: optimize
        // 1. If self.kind() == KIND_VEC, use Vec::extend
        for b in iter {
            self.put_u8(b);
        }
    }