unsafe fn full_buckets_indices(&self) -> FullBucketsIndices {
        // SAFETY:
        // 1. Since the caller of this function ensures that the control bytes
        //    are properly initialized and `self.ctrl(0)` points to the start
        //    of the array of control bytes, therefore: `ctrl` is valid for reads,
        //    properly aligned to `Group::WIDTH` and points to the properly initialized
        //    control bytes.
        // 2. The value of `items` is equal to the amount of data (values) added
        //    to the table.
        //
        //                         `ctrl` points here (to the start
        //                         of the first control byte `CT0`)
        //                          ∨
        // [Pad], T_n, ..., T1, T0, |CT0, CT1, ..., CT_n|, Group::WIDTH
        //                           \________  ________/
        //                                    \/
        //       `n = buckets - 1`, i.e. `RawTableInner::buckets() - 1`
        //
        // where: T0...T_n  - our stored data;
        //        CT0...CT_n - control bytes or metadata for `data`.
        let ctrl = NonNull::new_unchecked(self.ctrl(0).cast::<u8>());

        FullBucketsIndices {
            // Load the first group
            // SAFETY: See explanation above.
            current_group: Group::load_aligned(ctrl.as_ptr().cast())
                .match_full()
                .into_iter(),
            group_first_index: 0,
            ctrl,
            items: self.items,
        }
    }