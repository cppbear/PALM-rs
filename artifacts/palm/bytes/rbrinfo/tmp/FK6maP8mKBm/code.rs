unsafe fn shared_to_vec_impl(shared: *mut Shared, ptr: *const u8, len: usize) -> Vec<u8> {
    // Check that the ref_cnt is 1 (unique).
    //
    // If it is unique, then it is set to 0 with AcqRel fence for the same
    // reason in release_shared.
    //
    // Otherwise, we take the other branch and call release_shared.
    if (*shared)
        .ref_cnt
        .compare_exchange(1, 0, Ordering::AcqRel, Ordering::Relaxed)
        .is_ok()
    {
        // Deallocate the `Shared` instance without running its destructor.
        let shared = *Box::from_raw(shared);
        let shared = ManuallyDrop::new(shared);
        let buf = shared.buf;
        let cap = shared.cap;

        // Copy back buffer
        ptr::copy(ptr, buf, len);

        Vec::from_raw_parts(buf, len, cap)
    } else {
        let v = slice::from_raw_parts(ptr, len).to_vec();
        release_shared(shared);
        v
    }
}