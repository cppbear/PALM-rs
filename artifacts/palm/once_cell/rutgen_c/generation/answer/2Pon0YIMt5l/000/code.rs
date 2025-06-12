// Answer 0

#[test]
fn test_once_cell_invalid_sync_trait() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    fn share<T: Sync>(_: &T) {}

    // This should fail to compile if attempted to check Sync trait.
    // share(&once_cell::sync::OnceCell::<S>::new());
    // Uncommenting the above line should cause a compile-time error.
}

#[test]
fn test_lazy_invalid_sync_trait() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    fn share<T: Sync>(_: &T) {}

    // This should fail to compile if attempted to check Sync trait.
    // share(&once_cell::sync::Lazy::<S>::new(|| unimplemented!()));
    // Uncommenting the above line should cause a compile-time error.
}

