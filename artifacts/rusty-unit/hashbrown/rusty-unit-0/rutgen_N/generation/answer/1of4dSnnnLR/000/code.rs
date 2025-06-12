// Answer 0

#[test]
fn test_into_inner_valid() {
    use std::mem::ManuallyDrop;
    use std::ptr;

    struct ScopeGuard<T, F> {
        value: T,
        dropfn: F,
    }

    impl<T, F> ScopeGuard<T, F> {
        pub fn new(value: T, dropfn: F) -> Self {
            ScopeGuard { value, dropfn }
        }
    }

    fn into_inner<T, F>(guard: ScopeGuard<T, F>) -> T {
        let guard = ManuallyDrop::new(guard);
        unsafe {
            let value = ptr::read(&guard.value);
            let _ = ptr::read(&guard.dropfn);
            value
        }
    }

    let guard = ScopeGuard::new(42, || println!("Dropped"));
    let value = into_inner(guard);
    assert_eq!(value, 42);
}

#[test]
#[should_panic]
fn test_into_inner_invalid() {
    use std::mem::ManuallyDrop;
    use std::ptr;

    struct ScopeGuard<T, F> {
        value: T,
        dropfn: F,
    }

    impl<T, F> ScopeGuard<T, F> {
        pub fn new(value: T, dropfn: F) -> Self {
            ScopeGuard { value, dropfn }
        }
    }

    fn into_inner<T, F>(guard: ScopeGuard<T, F>) -> T {
        let guard = ManuallyDrop::new(guard);
        unsafe {
            let value = ptr::read(&guard.value);
            let _ = ptr::read(&guard.dropfn);
            value
        }
    }

    let guard = ScopeGuard::new(String::from("Test"), || panic!("Dropped"));
    let _value = into_inner(guard);
}

