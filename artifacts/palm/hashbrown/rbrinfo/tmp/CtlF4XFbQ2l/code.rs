unsafe fn get_many_unchecked_mut_inner<Q, const N: usize>(
        &mut self,
        ks: [&Q; N],
    ) -> [Option<&'_ mut (K, V)>; N]
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        let hashes = self.build_hashes_inner(ks);
        self.table
            .get_many_unchecked_mut(hashes, |i, (k, _)| ks[i].equivalent(k))
    }