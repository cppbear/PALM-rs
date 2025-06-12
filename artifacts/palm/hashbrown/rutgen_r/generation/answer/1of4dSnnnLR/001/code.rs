// Answer 0

#[test]
fn test_into_inner() {
    use std::ptr::ManuallyDrop;
    use std::ptr;

    struct TestGuard<T> {
        value: T,
        dropfn: Box<dyn Fn()>,
    }

    impl<T> TestGuard<T> {
        fn new(value: T, dropfn: Box<dyn Fn()>) -> Self {
            TestGuard { value, dropfn }
        }
    }

    let test_value = 42;
    let drop_fn_called = std::cell::Cell::new(false);
    let drop_fn = Box::new(move || {
        drop_fn_called.set(true);
    });

    let guard = TestGuard::new(test_value, drop_fn);

    let result = into_inner(guard);
    
    assert_eq!(result, 42);
    assert!(drop_fn_called.get());
}

#[test]
#[should_panic]
fn test_into_inner_panic_on_double_drop() {
    use std::ptr::ManuallyDrop;
    use std::ptr;

    struct TestGuard<T> {
        value: T,
        dropfn: Box<dyn Fn()>,
    }

    impl<T> TestGuard<T> {
        fn new(value: T, dropfn: Box<dyn Fn()>) -> Self {
            TestGuard { value, dropfn }
        }
    }

    let drop_fn_called = std::cell::Cell::new(false);
    let drop_fn = Box::new(move || {
        drop_fn_called.set(true);
    });

    let guard = TestGuard::new(10, drop_fn);

    let _result = into_inner(guard);
    let _result = into_inner(guard); // This should panic due to double drop
}

