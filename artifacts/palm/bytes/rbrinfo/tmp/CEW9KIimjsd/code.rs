unsafe fn shared_v_to_vec(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Vec<u8> {
    let shared: *mut Shared = data.load(Ordering::Relaxed).cast();

    if (*shared).is_unique() {
        let shared = &mut *shared;

        // Drop shared
        let mut vec = mem::replace(&mut shared.vec, Vec::new());
        release_shared(shared);

        // Copy back buffer
        ptr::copy(ptr, vec.as_mut_ptr(), len);
        vec.set_len(len);

        vec
    } else {
        let v = slice::from_raw_parts(ptr, len).to_vec();
        release_shared(shared);
        v
    }
}