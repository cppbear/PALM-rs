pub unsafe fn get_many_unchecked_mut<const N: usize>(
        &mut self,
        hashes: [u64; N],
        eq: impl FnMut(usize, &T) -> bool,
    ) -> [Option<&'_ mut T>; N] {
        let ptrs = self.get_many_mut_pointers(hashes, eq);
        ptrs.map(|ptr| ptr.map(|mut ptr| ptr.as_mut()))
    }