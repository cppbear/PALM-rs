// Answer 0

#[test]
fn test_once_box_with_sync_type() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    struct OnceBox<T> {
        ptr: *mut T,
    }

    impl<T> OnceBox<T> {
        fn new() -> OnceBox<T> {
            OnceBox {
                ptr: Box::into_raw(Box::new(unsafe { std::mem::zeroed() })),
            }
        }
    }

    fn share<T: Sync>(_: &T) {}

    let instance = OnceBox::<S>::new();
    share(&instance);
}

#[test]
#[should_panic]
fn test_once_box_with_non_sync_type() {
    struct NonSync;

    struct OnceBox<T> {
        ptr: *mut T,
    }

    impl<T> OnceBox<T> {
        fn new() -> OnceBox<T> {
            OnceBox {
                ptr: Box::into_raw(Box::new(unsafe { std::mem::zeroed() })),
            }
        }
    }

    fn share<T: Sync>(_: &T) {}

    let instance = OnceBox::<NonSync>::new();
    // This will panic because NonSync does not implement Sync.
    share(&instance);
}

