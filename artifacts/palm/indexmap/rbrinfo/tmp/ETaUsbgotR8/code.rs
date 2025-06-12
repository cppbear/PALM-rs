fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut K, &mut V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        if let Some(i) = self.get_index_of(key) {
            let entry = &mut self.as_entries_mut()[i];
            Some((i, &mut entry.key, &mut entry.value))
        } else {
            None
        }
    }