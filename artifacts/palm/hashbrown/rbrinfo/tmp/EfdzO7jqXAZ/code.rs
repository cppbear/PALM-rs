unsafe fn iter<T>(&self) -> RawIter<T> {
        // SAFETY:
        // 1. Since the caller of this function ensures that the control bytes
        //    are properly initialized and `self.data_end()` points to the start
        //    of the array of control bytes, therefore: `ctrl` is valid for reads,
        //    properly aligned to `Group::WIDTH` and points to the properly initialized
        //    control bytes.
        // 2. `data` bucket index in the table is equal to the `ctrl` index (i.e.
        //    equal to zero).
        // 3. We pass the exact value of buckets of the table to the function.
        //
        //                         `ctrl` points here (to the start
        //                         of the first control byte `CT0`)
        //                          ∨
        // [Pad], T_n, ..., T1, T0, |CT0, CT1, ..., CT_n|, CTa_0, CTa_1, ..., CTa_m
        //                           \________  ________/
        //                                    \/
        //       `n = buckets - 1`, i.e. `RawTableInner::buckets() - 1`
        //
        // where: T0...T_n  - our stored data;
        //        CT0...CT_n - control bytes or metadata for `data`.
        //        CTa_0...CTa_m - additional control bytes, where `m = Group::WIDTH - 1` (so that the search
        //                        with loading `Group` bytes from the heap works properly, even if the result
        //                        of `h1(hash) & self.bucket_mask` is equal to `self.bucket_mask`). See also
        //                        `RawTableInner::set_ctrl` function.
        //
        // P.S. `h1(hash) & self.bucket_mask` is the same as `hash as usize % self.buckets()` because the number
        // of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
        let data = Bucket::from_base_index(self.data_end(), 0);
        RawIter {
            // SAFETY: See explanation above
            iter: RawIterRange::new(self.ctrl.as_ptr(), data, self.buckets()),
            items: self.items,
        }
    }