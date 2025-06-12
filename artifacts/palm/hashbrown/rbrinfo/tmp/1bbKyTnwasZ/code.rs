pub(crate) fn find_or_find_insert_slot<Q>(
        &mut self,
        hash: u64,
        key: &Q,
    ) -> Result<Bucket<(K, V)>, crate::raw::InsertSlot>
    where
        Q: Equivalent<K> + ?Sized,
    {
        self.table.find_or_find_insert_slot(
            hash,
            equivalent_key(key),
            make_hasher(&self.hash_builder),
        )
    }