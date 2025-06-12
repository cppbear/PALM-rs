pub unsafe fn insert_unique_unchecked(&mut self, k: K, v: V) -> (&K, &mut V) {
        let hash = make_hash::<K, S>(&self.hash_builder, &k);
        let bucket = self
            .table
            .insert(hash, (k, v), make_hasher::<_, V, S>(&self.hash_builder));
        let (k_ref, v_ref) = unsafe { bucket.as_mut() };
        (k_ref, v_ref)
    }