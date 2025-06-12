// Answer 0

#[test]
#[should_panic]
fn test_sync_on_once_box() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    fn share<T: Sync>(_: &T) {}

    // Attempt to create an OnceBox with S and share it, which should panic at compile time
    share(&once_cell::race::OnceBox::<S>::new());
}

#[test]
fn test_no_panic_with_valid_sync() {
    struct T;
    unsafe impl Sync for T {}

    fn share<T: Sync>(_: &T) {}

    // Here we create a valid type that implements Sync and should not panic
    share(&once_cell::race::OnceBox::<T>::new());
}

