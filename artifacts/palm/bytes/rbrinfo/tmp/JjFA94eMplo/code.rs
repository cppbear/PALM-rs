unsafe fn shared_to_mut_impl(shared: *mut Shared, ptr: *const u8, len: usize) -> BytesMut {
    // The goal is to check if the current handle is the only handle
    // that currently has access to the buffer. This is done by
    // checking if the `ref_cnt` is currently 1.
    //
    // The `Acquire` ordering synchronizes with the `Release` as
    // part of the `fetch_sub` in `release_shared`. The `fetch_sub`
    // operation guarantees that any mutations done in other threads
    // are ordered before the `ref_cnt` is decremented. As such,
    // this `Acquire` will guarantee that those mutations are
    // visible to the current thread.
    //
    // Otherwise, we take the other branch, copy the data and call `release_shared`.
    if (*shared).ref_cnt.load(Ordering::Acquire) == 1 {
        // Deallocate the `Shared` instance without running its destructor.
        let shared = *Box::from_raw(shared);
        let shared = ManuallyDrop::new(shared);
        let buf = shared.buf;
        let cap = shared.cap;

        // Rebuild Vec
        let off = offset_from(ptr, buf);
        let v = Vec::from_raw_parts(buf, len + off, cap);

        let mut b = BytesMut::from_vec(v);
        b.advance_unchecked(off);
        b
    } else {
        // Copy the data from Shared in a new Vec, then release it
        let v = slice::from_raw_parts(ptr, len).to_vec();
        release_shared(shared);
        BytesMut::from_vec(v)
    }
}