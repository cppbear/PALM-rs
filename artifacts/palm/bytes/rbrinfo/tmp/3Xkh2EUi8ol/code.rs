unsafe fn owned_clone(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Bytes {
    let owned = data.load(Ordering::Relaxed);
    let ref_cnt = &(*owned.cast::<OwnedLifetime>()).ref_cnt;
    let old_cnt = ref_cnt.fetch_add(1, Ordering::Relaxed);
    if old_cnt > usize::MAX >> 1 {
        crate::abort()
    }

    Bytes {
        ptr,
        len,
        data: AtomicPtr::new(owned as _),
        vtable: &OWNED_VTABLE,
    }
}