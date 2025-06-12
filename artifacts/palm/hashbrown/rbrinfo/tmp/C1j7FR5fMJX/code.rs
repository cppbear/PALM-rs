pub unsafe fn bucket(&self, index: usize) -> Bucket<T> {
        // If mem::size_of::<T>() != 0 then return a pointer to the `element` in the `data part` of the table
        // (we start counting from "0", so that in the expression T[n], the "n" index actually one less than
        // the "buckets" number of our `RawTable`, i.e. "n = RawTable::buckets() - 1"):
        //
        //           `table.bucket(3).as_ptr()` returns a pointer that points here in the `data`
        //           part of the `RawTable`, i.e. to the start of T3 (see `Bucket::as_ptr`)
        //                  |
        //                  |               `base = self.data_end()` points here
        //                  |               (to the start of CT0 or to the end of T0)
        //                  v                 v
        // [Pad], T_n, ..., |T3|, T2, T1, T0, |CT0, CT1, CT2, CT3, ..., CT_n, CTa_0, CTa_1, ..., CTa_m
        //                     ^                                              \__________  __________/
        //        `table.bucket(3)` returns a pointer that points                        \/
        //         here in the `data` part of the `RawTable` (to              additional control bytes
        //         the end of T3)                                              `m = Group::WIDTH - 1`
        //
        // where: T0...T_n  - our stored data;
        //        CT0...CT_n - control bytes or metadata for `data`;
        //        CTa_0...CTa_m - additional control bytes (so that the search with loading `Group` bytes from
        //                        the heap works properly, even if the result of `h1(hash) & self.table.bucket_mask`
        //                        is equal to `self.table.bucket_mask`). See also `RawTableInner::set_ctrl` function.
        //
        // P.S. `h1(hash) & self.table.bucket_mask` is the same as `hash as usize % self.buckets()` because the number
        // of buckets is a power of two, and `self.table.bucket_mask = self.buckets() - 1`.
        debug_assert_ne!(self.table.bucket_mask, 0);
        debug_assert!(index < self.buckets());
        Bucket::from_base_index(self.data_end(), index)
    }