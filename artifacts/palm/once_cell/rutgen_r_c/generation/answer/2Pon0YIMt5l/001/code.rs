// Answer 0

#[test]
fn test_once_cell_sync_fail() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    fn share<T: Sync>(_: &T) {}

    // This will fail to compile as expected because S is not safe to share
    let once_cell = once_cell::sync::OnceCell::<S>::new();
    share(&once_cell); // should trigger a compile failure
}

#[test]
fn test_lazy_sync_fail() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    fn share<T: Sync>(_: &T) {}

    // This will fail to compile as expected because S is not safe to share
    let lazy = once_cell::sync::Lazy::<S>::new(|| unimplemented!());
    share(&lazy); // should trigger a compile failure
}

