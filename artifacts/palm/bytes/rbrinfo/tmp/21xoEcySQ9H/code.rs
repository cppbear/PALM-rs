unsafe fn static_drop(_: &mut AtomicPtr<()>, _: *const u8, _: usize) {
    // nothing to drop for &'static [u8]
}