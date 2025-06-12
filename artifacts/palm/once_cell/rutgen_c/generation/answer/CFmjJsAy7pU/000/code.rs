// Answer 0

#[test]
fn test_once_box_with_sync() {
    struct S(*mut ());
    unsafe impl Sync for S {}

    struct OnceBox<T>(Box<T>, PhantomData<T>);

    impl<T> OnceBox<T> {
        fn new(value: T) -> Self {
            OnceBox(Box::new(value), PhantomData)
        }
    }

    fn share<T: Sync>(_: &T) {}

    let s = S(ptr::null_mut());
    let once_box = OnceBox::new(s);
    share(&once_box);
}

#[test]
#[should_panic]
fn test_once_box_without_sync() {
    struct NonSync;

    struct OnceBox<T>(Box<T>, PhantomData<T>);

    impl<T> OnceBox<T> {
        fn new(value: T) -> Self {
            OnceBox(Box::new(value), PhantomData)
        }
    }

    fn share<T: Sync>(_: &T) {}

    let non_sync_instance = NonSync;
    let once_box = OnceBox::new(non_sync_instance);
    share(&once_box);
}

