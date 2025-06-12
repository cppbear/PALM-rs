unsafe fn owned_drop_impl(owned: *mut ()) {
    let lifetime = owned.cast::<OwnedLifetime>();
    let ref_cnt = &(*lifetime).ref_cnt;

    let old_cnt = ref_cnt.fetch_sub(1, Ordering::Release);
    debug_assert!(
        old_cnt > 0 && old_cnt <= usize::MAX >> 1,
        "expected non-zero refcount and no underflow"
    );
    if old_cnt != 1 {
        return;
    }
    ref_cnt.load(Ordering::Acquire);

    let drop_fn = &(*lifetime).drop;
    drop_fn(owned)
}