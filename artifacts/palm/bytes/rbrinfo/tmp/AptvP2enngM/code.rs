unsafe fn promotable_odd_drop(data: &mut AtomicPtr<()>, ptr: *const u8, len: usize) {
    data.with_mut(|shared| {
        let shared = *shared;
        let kind = shared as usize & KIND_MASK;

        if kind == KIND_ARC {
            release_shared(shared.cast());
        } else {
            debug_assert_eq!(kind, KIND_VEC);

            free_boxed_slice(shared.cast(), ptr, len);
        }
    });
}