fn get_inner_mut<Q>(&mut self, k: &Q) -> Option<&mut (K, V)>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        if self.table.is_empty() {
            None
        } else {
            let hash = make_hash::<Q, S>(&self.hash_builder, k);
            self.table.get_mut(hash, equivalent_key(k))
        }
    }