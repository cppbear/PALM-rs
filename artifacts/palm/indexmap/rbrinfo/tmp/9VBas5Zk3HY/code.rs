fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut T)>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        match self.map.get_full_mut2(value) {
            Some((index, value, ())) => Some((index, value)),
            None => None,
        }
    }