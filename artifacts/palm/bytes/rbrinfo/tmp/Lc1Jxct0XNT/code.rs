unsafe fn shared_v_to_mut(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> BytesMut {
    let shared: *mut Shared = data.load(Ordering::Relaxed).cast();

    if (*shared).is_unique() {
        let shared = &mut *shared;

        // The capacity is always the original capacity of the buffer
        // minus the offset from the start of the buffer
        let v = &mut shared.vec;
        let v_capacity = v.capacity();
        let v_ptr = v.as_mut_ptr();
        let offset = offset_from(ptr as *mut u8, v_ptr);
        let cap = v_capacity - offset;

        let ptr = vptr(ptr as *mut u8);

        BytesMut {
            ptr,
            len,
            cap,
            data: shared,
        }
    } else {
        let v = slice::from_raw_parts(ptr, len).to_vec();
        release_shared(shared);
        BytesMut::from_vec(v)
    }
}