// Answer 0

#[test]
fn test_once_box_sync() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    fn share<T: Sync>(_: &T) {}

    let once_box = once_cell::race::OnceBox::<S>::new();
    share(&once_box);
}

