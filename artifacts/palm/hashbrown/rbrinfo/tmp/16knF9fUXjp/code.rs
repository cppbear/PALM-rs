pub fn get_many_key_value_mut<Q, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> [Option<(&'_ K, &'_ mut V)>; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        self.get_many_mut_inner(ks)
            .map(|res| res.map(|(k, v)| (&*k, v)))
    }