unsafe fn shallow_clone_arc(shared: *mut Shared, ptr: *const u8, len: usize) -> Bytes {
    let old_size = (*shared).ref_cnt.fetch_add(1, Ordering::Relaxed);

    if old_size > usize::MAX >> 1 {
        crate::abort();
    }

    Bytes {
        ptr,
        len,
        data: AtomicPtr::new(shared as _),
        vtable: &SHARED_VTABLE,
    }
}