fn get_inner<Q>(&self, k: &Q) -> Option<&(K, V)>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        if self.table.is_empty() {
            None
        } else {
            let hash = make_hash::<Q, S>(&self.hash_builder, k);
            self.table.get(hash, equivalent_key(k))
        }
    }