pub fn insert_with_hasher<H>(
        self,
        hash: u64,
        key: K,
        value: V,
        hasher: H,
    ) -> (&'a mut K, &'a mut V)
    where
        H: Fn(&K) -> u64,
    {
        let &mut (ref mut k, ref mut v) = self
            .table
            .insert_entry(hash, (key, value), |x| hasher(&x.0));
        (k, v)
    }