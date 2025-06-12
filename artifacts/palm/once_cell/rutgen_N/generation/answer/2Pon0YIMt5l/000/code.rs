// Answer 0

#[test]
fn test_once_cell_sync_fail() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    fn share<T: Sync>(_: &T) {}

    // This should fail to compile, as OnceCell<S> cannot be Sync
    let _cell = once_cell::sync::OnceCell::<S>::new();
    share(&_cell);
}

#[test]
fn test_lazy_sync_fail() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    fn share<T: Sync>(_: &T) {}

    // This should fail to compile, as Lazy<S> cannot be Sync
    let _lazy = once_cell::sync::Lazy::<S>::new(|| unimplemented!());
    share(&_lazy);
}

