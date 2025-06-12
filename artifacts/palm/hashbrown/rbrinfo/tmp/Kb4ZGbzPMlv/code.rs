pub fn data_end(&self) -> NonNull<T> {
        //                        `self.table.ctrl.cast()` returns pointer that
        //                        points here (to the end of `T0`)
        //                          ∨
        // [Pad], T_n, ..., T1, T0, |CT0, CT1, ..., CT_n|, CTa_0, CTa_1, ..., CTa_m
        //                           \________  ________/
        //                                    \/
        //       `n = buckets - 1`, i.e. `RawTable::buckets() - 1`
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
        self.table.ctrl.cast()
    }