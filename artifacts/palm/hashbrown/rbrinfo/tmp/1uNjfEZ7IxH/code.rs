unsafe fn get_many_mut_pointers<const N: usize>(
        &mut self,
        hashes: [u64; N],
        mut eq: impl FnMut(usize, &T) -> bool,
    ) -> [Option<NonNull<T>>; N] {
        array::from_fn(|i| {
            self.find(hashes[i], |k| eq(i, k))
                .map(|cur| cur.as_non_null())
        })
    }