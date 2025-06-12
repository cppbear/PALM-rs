unsafe fn to_base_index(&self, base: NonNull<T>) -> usize {
        // If mem::size_of::<T>() != 0 then return an index under which we used to store the
        // `element` in the data part of the table (we start counting from "0", so
        // that in the expression T[last], the "last" index actually is one less than the
        // "buckets" number in the table, i.e. "last = RawTableInner.bucket_mask").
        // For example for 5th element in table calculation is performed like this:
        //
        //                        mem::size_of::<T>()
        //                          |
        //                          |         `self = from_base_index(base, 5)` that returns pointer
        //                          |         that points here in the data part of the table
        //                          |         (to the end of T5)
        //                          |           |                    `base: NonNull<T>` must point here
        //                          v           |                    (to the end of T0 or to the start of C0)
        //                        /???\         v                      v
        // [Padding], Tlast, ..., |T10|, ..., T5|, T4, T3, T2, T1, T0, |C0, C1, C2, C3, C4, C5, ..., C10, ..., Clast
        //                                      \__________  __________/
        //                                                 \/
        //                                     `bucket.to_base_index(base)` = 5
        //                                     (base.as_ptr() as usize - self.ptr.as_ptr() as usize) / mem::size_of::<T>()
        //
        // where: T0...Tlast - our stored data; C0...Clast - control bytes or metadata for data.
        if T::IS_ZERO_SIZED {
            // this can not be UB
            self.ptr.as_ptr() as usize - 1
        } else {
            offset_from(base.as_ptr(), self.ptr.as_ptr())
        }
    }