unsafe fn owned_drop(data: &mut AtomicPtr<()>, _ptr: *const u8, _len: usize) {
    let owned = data.load(Ordering::Relaxed);
    owned_drop_impl(owned);
}