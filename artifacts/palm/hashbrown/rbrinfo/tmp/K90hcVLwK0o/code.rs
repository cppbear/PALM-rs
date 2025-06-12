pub fn remove_entry<Q>(&mut self, k: &Q) -> Option<(K, V)>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        let hash = make_hash::<Q, S>(&self.hash_builder, k);
        self.table.remove_entry(hash, equivalent_key(k))
    }