// Answer 0

#[test]
fn test_once_cell_sync_failure() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    fn share<T: Sync>(_: &T) {}

    // This will trigger a compile_fail as expected.  
    let once_cell = once_cell::sync::OnceCell::<S>::new();
    share(&once_cell);
}

#[test]
#[should_panic]
fn test_lazy_sync_failure() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    fn share<T: Sync>(_: &T) {}

    // This will also trigger a compile_fail as expected.
    let lazy = once_cell::sync::Lazy::<S>::new(|| unimplemented!());
    share(&lazy);
}

