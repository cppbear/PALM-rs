unsafe fn promotable_to_vec(
    data: &AtomicPtr<()>,
    ptr: *const u8,
    len: usize,
    f: fn(*mut ()) -> *mut u8,
) -> Vec<u8> {
    let shared = data.load(Ordering::Acquire);
    let kind = shared as usize & KIND_MASK;

    if kind == KIND_ARC {
        shared_to_vec_impl(shared.cast(), ptr, len)
    } else {
        // If Bytes holds a Vec, then the offset must be 0.
        debug_assert_eq!(kind, KIND_VEC);

        let buf = f(shared);

        let cap = offset_from(ptr, buf) + len;

        // Copy back buffer
        ptr::copy(ptr, buf, len);

        Vec::from_raw_parts(buf, len, cap)
    }
}