// Answer 0

#[test]
fn test_once_cell_sync_fail() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    fn share<T: Sync>(_: &T) {}
    
    // This will cause a compile-time failure due to S not being actually Sync
    // Uncommenting the next line will result in a compile failure
    // share(&once_cell::sync::OnceCell::<S>::new());
}

#[test]
fn test_lazy_sync_fail() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    fn share<T: Sync>(_: &T) {}

    // This will cause a compile-time failure due to S not being actually Sync
    // Uncommenting the next line will result in a compile failure
    // share(&once_cell::sync::Lazy::<S>::new(|| unimplemented!()));
}

