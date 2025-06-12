unsafe fn promote_to_shared(&mut self, ref_cnt: usize) {
        debug_assert_eq!(self.kind(), KIND_VEC);
        debug_assert!(ref_cnt == 1 || ref_cnt == 2);

        let original_capacity_repr =
            (self.data as usize & ORIGINAL_CAPACITY_MASK) >> ORIGINAL_CAPACITY_OFFSET;

        // The vec offset cannot be concurrently mutated, so there
        // should be no danger reading it.
        let off = (self.data as usize) >> VEC_POS_OFFSET;

        // First, allocate a new `Shared` instance containing the
        // `Vec` fields. It's important to note that `ptr`, `len`,
        // and `cap` cannot be mutated without having `&mut self`.
        // This means that these fields will not be concurrently
        // updated and since the buffer hasn't been promoted to an
        // `Arc`, those three fields still are the components of the
        // vector.
        let shared = Box::new(Shared {
            vec: rebuild_vec(self.ptr.as_ptr(), self.len, self.cap, off),
            original_capacity_repr,
            ref_count: AtomicUsize::new(ref_cnt),
        });

        let shared = Box::into_raw(shared);

        // The pointer should be aligned, so this assert should
        // always succeed.
        debug_assert_eq!(shared as usize & KIND_MASK, KIND_ARC);

        self.data = shared;
    }