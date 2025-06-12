pub fn get_many_mut<const N: usize>(
        &mut self,
        hashes: [u64; N],
        eq: impl FnMut(usize, &T) -> bool,
    ) -> [Option<&'_ mut T>; N] {
        unsafe {
            let ptrs = self.get_many_mut_pointers(hashes, eq);

            for (i, cur) in ptrs.iter().enumerate() {
                if cur.is_some() && ptrs[..i].contains(cur) {
                    panic!("duplicate keys found");
                }
            }
            // All bucket are distinct from all previous buckets so we're clear to return the result
            // of the lookup.

            ptrs.map(|ptr| ptr.map(|mut ptr| ptr.as_mut()))
        }
    }