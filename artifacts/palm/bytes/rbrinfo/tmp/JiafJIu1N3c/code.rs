unsafe fn promotable_to_mut(
    data: &AtomicPtr<()>,
    ptr: *const u8,
    len: usize,
    f: fn(*mut ()) -> *mut u8,
) -> BytesMut {
    let shared = data.load(Ordering::Acquire);
    let kind = shared as usize & KIND_MASK;

    if kind == KIND_ARC {
        shared_to_mut_impl(shared.cast(), ptr, len)
    } else {
        // KIND_VEC is a view of an underlying buffer at a certain offset.
        // The ptr + len always represents the end of that buffer.
        // Before truncating it, it is first promoted to KIND_ARC.
        // Thus, we can safely reconstruct a Vec from it without leaking memory.
        debug_assert_eq!(kind, KIND_VEC);

        let buf = f(shared);
        let off = offset_from(ptr, buf);
        let cap = off + len;
        let v = Vec::from_raw_parts(buf, cap, cap);

        let mut b = BytesMut::from_vec(v);
        b.advance_unchecked(off);
        b
    }
}