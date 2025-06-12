unsafe fn shared_v_clone(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Bytes {
    let shared = data.load(Ordering::Relaxed) as *mut Shared;
    increment_shared(shared);

    let data = AtomicPtr::new(shared as *mut ());
    Bytes::with_vtable(ptr, len, data, &SHARED_VTABLE)
}