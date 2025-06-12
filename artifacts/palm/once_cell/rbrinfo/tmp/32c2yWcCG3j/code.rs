pub unsafe fn get_unchecked(&self) -> NonZeroUsize {
        #[inline(always)]
        fn as_const_ptr(r: &AtomicUsize) -> *const usize {
            use core::mem::align_of;

            let p: *const AtomicUsize = r;
            // SAFETY: "This type has the same size and bit validity as
            // the underlying integer type, usize. However, the alignment of
            // this type is always equal to its size, even on targets where
            // usize has a lesser alignment."
            const _ALIGNMENT_COMPATIBLE: () =
                assert!(align_of::<AtomicUsize>() % align_of::<usize>() == 0);
            p.cast::<usize>()
        }

        // TODO(MSRV-1.70): Use `AtomicUsize::as_ptr().cast_const()`
        // See https://github.com/rust-lang/rust/issues/138246.
        let p = as_const_ptr(&self.inner);

        // SAFETY: The caller is responsible for ensuring that the value
        // was initialized and that the contents have been acquired by
        // this thread. Assuming that, we can assume there will be no
        // conflicting writes to the value since the value will never
        // change once initialized. This relies on the statement in
        // https://doc.rust-lang.org/1.83.0/core/sync/atomic/ that "(A
        // `compare_exchange` or `compare_exchange_weak` that does not
        // succeed is not considered a write."
        let val = unsafe { p.read() };

        // SAFETY: The caller is responsible for ensuring the value is
        // initialized and thus not zero.
        unsafe { NonZeroUsize::new_unchecked(val) }
    }